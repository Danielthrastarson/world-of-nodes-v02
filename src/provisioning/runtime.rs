use super::*;

/// Time to next awake performance/critical threshold; positive even at a boundary.
pub(crate) fn awake_threshold(n: Time, f: Time) -> Time {
    let nutrition = if n > 6 * HOUR {
        n - 6 * HOUR
    } else if n > 0 {
        n
    } else {
        Time::MAX / 4
    };
    let fatigue = [12, 16, 20, 22, 24]
        .into_iter()
        .map(|h| h * HOUR)
        .find(|&x| x > f)
        .map_or(Time::MAX / 4, |x| x - f);
    nutrition.min(fatigue)
}
impl World {
    pub fn apply_intents(&mut self, mut intents: Vec<(usize, Intent)>) -> Result<(), String> {
        intents.sort_by_key(|(id, _)| *id);
        if intents.iter().any(|(id, _)| *id >= self.agents.len())
            || intents.windows(2).any(|w| w[0].0 == w[1].0)
        {
            return Err("invalid/duplicate actor in intent batch".into());
        }
        for (id, intent) in intents {
            if let Err(error) = self.execute(id, intent.clone()) {
                self.log(Some(id), "Rejected", format!("{intent:?}: {error}"));
            }
        }
        self.admit();
        self.audit()
    }
    pub(super) fn cancel_action(&mut self, id: usize, why: &str) {
        if matches!(
            self.agents[id].action,
            Action::Queued { .. }
                | Action::Gather { .. }
                | Action::Sleep { .. }
                | Action::Farm { .. }
                | Action::BuildCart { .. }
                | Action::BuildCrossing { .. }
        ) {
            if matches!(self.agents[id].action, Action::Farm { .. }) {
                self.farm.interruptions += 1;
            }
            self.log(
                Some(id),
                "Interrupted",
                format!("{why}; was={:?}", self.agents[id].action),
            );
            self.agents[id].action = Action::Idle;
            self.gather_communal.remove(&id);
        }
    }
    fn cancel_job(&mut self, id: usize, reason: &str) {
        if let Some(job) = self.agents[id].job.take() {
            self.metrics.cancellations += 1;
            if reason.contains("survival-infeasible") {
                self.metrics.infeasible_abandons += 1;
            }
            // Only the physical board can be informed locally.
            if self.agents[id].location == Location::Node(job.destination)
                && let Some(b) = self.boards.iter_mut().find(|b| b.node == job.destination)
            {
                b.claims.retain(|c| c.agent != id);
            }
            self.log(
                Some(id),
                "JobCancelled",
                format!(
                    "request={}; reason={reason}; remote board unchanged",
                    job.request
                ),
            );
        }
    }
    fn local_batch(&self, id: usize, batch: usize) -> Result<Batch, String> {
        let b = self.goods.batches.get(batch).ok_or("unknown batch")?;
        let local = match self.agents[id].location {
            Location::Node(n) => {
                matches!(b.holder,Holder::Deposit(x)|Holder::Loose(x)|Holder::Commons(x)if n==x)
                    || self.carried_by(id, b.holder)
                    || self.accessible_cart(id, b.holder)
            }
            _ => false,
        };
        if !local || b.quantity == 0 {
            return Err("batch not physically accessible".into());
        }
        if b.owner != Owner::None
            && b.owner != Owner::Agent(id)
            && !(self.config.communal && b.owner == Owner::Commons(0))
        {
            return Err("ownership forbids access".into());
        }
        Ok(b.clone())
    }
    fn execute(&mut self, id: usize, intent: Intent) -> Result<(), String> {
        if self.agents[id].action == Action::Dead {
            return Err("dead agents cannot act".into());
        }
        if matches!(intent, Intent::Wait) {
            if matches!(self.agents[id].action, Action::Sleep { until } if until <= self.now) {
                self.agents[id].action = Action::Idle;
            }
            return Ok(());
        }
        let Location::Node(node) = self.agents[id].location else {
            return Err("edge traversal is atomic".into());
        };
        match intent {
            Intent::OfferBarter {
                batch,
                quantity,
                want,
                want_quantity,
            } => {
                self.post_barter(id, batch, quantity, want, want_quantity)?;
            }
            Intent::AcceptBarter { offer, payment } => {
                if let Err(reason) = self.accept_barter(id, offer, payment) {
                    self.record_barter_failure(&reason);
                    return Err(reason);
                }
            }
            Intent::RevokeBarter { offer } => self.revoke_barter(id, offer)?,
            Intent::ContributeCrossing {
                project,
                batch,
                quantity,
            } => self.donate_crossing(id, project, batch, quantity)?,
            Intent::BuildCrossing { project } => self.begin_crossing(id, project)?,
            Intent::BuildCart => self.begin_cart(id)?,
            Intent::HitchCart { cart } => self.hitch_cart(id, cart)?,
            Intent::ParkCart => self.park_cart(id)?,
            Intent::Drop { batch, quantity } => {
                let b = self.local_batch(id, batch)?;
                if !self.carried_by(id, b.holder)
                    || b.owner != Owner::Agent(id)
                    || quantity == 0
                    || quantity > b.quantity
                {
                    return Err("invalid personal drop".into());
                }
                self.cancel_action(id, "drop personal goods");
                self.goods
                    .move_part(batch, quantity, b.owner, Holder::Loose(node));
                self.log(Some(id), "Dropped", format!("batch={batch} q={quantity}"));
            }
            Intent::Cultivate { plot } => self.begin_farm(id, plot)?,
            Intent::Wait => {}
            Intent::CancelAction => {
                self.cancel_action(id, "cancel invalid research route");
                self.agents[id].goal = None;
            }
            Intent::Eat { batch, quantity } => {
                let b = self.local_batch(id, batch)?;
                if b.material != Material::Food
                    || b.expires_at() <= self.now
                    || matches!(b.holder, Holder::Deposit(_))
                    || quantity == 0
                    || quantity > b.quantity
                    || quantity > goods::headroom(self.agents[id].nutrition, b.owner)
                {
                    return Err("inedible/unharvested stock or ration limit".into());
                }
                self.cancel_action(id, "eat accessible food");
                self.goods.batches[batch].quantity -= quantity;
                self.goods.consumed += quantity as u128;
                self.metrics.consumed += quantity;
                if b.owner == Owner::Commons(0) && self.carried_by(id, b.holder) {
                    self.diagnostics.cargo_eaten(id, quantity);
                }
                self.agents[id].nutrition += quantity as Time * HOUR;
                self.agents[id].starvation = 0;
                self.agents[id].consumed += quantity;
                self.log(
                    Some(id),
                    "Consumed",
                    format!(
                        "batch={batch} q={quantity} owner={:?} nutrition_ms={}",
                        b.owner, self.agents[id].nutrition
                    ),
                );
            }
            Intent::Gather { batch, communal } => {
                if matches!(self.agents[id].action,Action::Gather{batch:b,..}if b==batch) {
                    return Ok(());
                }
                let b = self.local_batch(id, batch)?;
                if !(b.material == Material::Food
                    || ((self.config.capital.is_some() || self.config.infrastructure.is_some())
                        && b.material == Material::Wood
                        && !communal))
                    || b.expires_at() <= self.now
                    || !matches!(b.holder, Holder::Deposit(_))
                    || self.reserved(batch) >= b.quantity
                    || self.carrier_load(id) >= self.carrier_capacity(id)
                {
                    return Err("gather inputs/capacity unavailable".into());
                }
                if communal
                    && !self.agents[id]
                        .job
                        .as_ref()
                        .is_some_and(|j| j.acquired < j.target)
                {
                    return Err("communal gathering requires accepted unmet job".into());
                }
                self.cancel_action(id, "new gathering action");
                self.agents[id].action = Action::Gather {
                    batch,
                    remaining: 2000 * SCALE,
                };
                // Ownership on completion follows the declared action, not a later inference.
                self.gather_communal.insert(id, communal);
                self.log(
                    Some(id),
                    "GatherStarted",
                    format!("batch={batch} reserved=1 communal={communal}"),
                );
            }
            Intent::LoadCargo { batch, quantity } => {
                let b = self.local_batch(id, batch)?;
                let job = self.agents[id]
                    .job
                    .as_ref()
                    .ok_or("cargo loading requires an accepted job")?;
                if node == job.destination
                    || quantity == 0
                    || quantity > b.quantity
                    || quantity > job.target.saturating_sub(job.acquired)
                    || self.carrier_load(id) + quantity > self.carrier_capacity(id)
                    || b.material != Material::Food
                    || b.expires_at() <= self.now
                    || !matches!(b.holder, Holder::Loose(_) | Holder::Commons(_))
                {
                    return Err("cargo loading precondition".into());
                }
                self.cancel_action(id, "load communal cargo");
                self.load_carrier(id, batch, quantity, Owner::Commons(0));
                self.agents[id].job.as_mut().unwrap().acquired += quantity;
                self.log(
                    Some(id),
                    "CargoLoaded",
                    format!("batch={batch} q={quantity} owner=Commons(0)"),
                );
            }
            Intent::Pickup { batch, quantity } => {
                let b = self.local_batch(id, batch)?;
                if !matches!(b.holder, Holder::Loose(_))
                    || matches!(b.owner, Owner::Commons(_))
                    || quantity == 0
                    || quantity > b.quantity
                    || self.carrier_load(id) + quantity > self.carrier_capacity(id)
                {
                    return Err("invalid personal pickup".into());
                }
                self.cancel_action(id, "pickup");
                self.load_carrier(id, batch, quantity, Owner::Agent(id));
                self.log(Some(id), "Pickup", format!("batch={batch} q={quantity}"));
            }
            Intent::Travel { edge, target, food } => {
                let e = self.config.edges.get(edge).ok_or("unknown edge")?;
                let to = if e.a == node {
                    e.b
                } else if e.b == node {
                    e.a
                } else {
                    return Err("nonlocal edge".into());
                };
                if target >= self.config.nodes {
                    return Err("invalid goal".into());
                }
                if matches!(self.agents[id].action,Action::Queued{edge:x,..}if x==edge) {
                    return Ok(());
                }
                self.cancel_action(id, "queue for route");
                self.agents[id].goal = Some((target, food));
                self.agents[id].action = Action::Queued {
                    edge,
                    to,
                    joined: self.now,
                };
                self.log(
                    Some(id),
                    "Queued",
                    format!("edge={edge} target={target} remembered_food={food}"),
                );
            }
            Intent::Sleep => {
                if matches!(self.agents[id].action,Action::Sleep{until}if until>self.now) {
                    return Ok(());
                }
                self.cancel_action(id, "sleep");
                self.agents[id].action = Action::Sleep {
                    until: self.now + MINUTE,
                };
                self.log(
                    Some(id),
                    "SleepStarted",
                    format!("fatigue_ms={}", self.agents[id].fatigue),
                );
            }
            Intent::Claim { board, quantity } => {
                if !self.config.communal
                    || board != node
                    || self.agents[id].job.is_some()
                    || quantity == 0
                    || quantity > self.carrier_capacity(id) - self.carrier_load(id)
                {
                    return Err("claim precondition".into());
                }
                let b = self
                    .boards
                    .iter_mut()
                    .find(|b| b.node == node)
                    .ok_or("no local board")?;
                if quantity > b.unclaimed(self.now) {
                    return Err("demand already claimed".into());
                }
                b.claims.push(Claim {
                    agent: id,
                    quantity,
                    expires_at: self.now + 12 * HOUR,
                });
                self.cancel_action(id, "accept local request");
                self.agents[id].job = Some(Job {
                    request: board,
                    destination: board,
                    target: quantity,
                    acquired: 0,
                });
                self.metrics.jobs += 1;
                self.log(
                    Some(id),
                    "JobAccepted",
                    format!(
                        "board={board} q={quantity} lease_expires={}",
                        self.now + 12 * HOUR
                    ),
                );
            }
            Intent::Deliver { batch, quantity } => {
                let b = self.local_batch(id, batch)?;
                if !self.carried_by(id, b.holder)
                    || b.owner != Owner::Commons(0)
                    || b.material != Material::Food
                    || quantity == 0
                    || quantity > b.quantity
                {
                    return Err("delivery requires physical communal cargo".into());
                }
                let job = self.agents[id].job.clone().ok_or("no delivery intention")?;
                if node != job.destination {
                    return Err("not at delivery destination".into());
                }
                self.cancel_action(id, "delivery");
                self.goods
                    .move_part(batch, quantity, b.owner, Holder::Commons(node));
                self.metrics.delivered += quantity;
                if self.config.boards.first() == Some(&node) {
                    self.request_backlog = self.request_backlog.saturating_sub(quantity);
                }
                if let Some(board) = self.boards.iter_mut().find(|b| b.node == node) {
                    board.quantity = board.quantity.saturating_sub(quantity);
                    board.claims.retain(|c| c.agent != id);
                }
                // New cart payload may span both backpack and cart containers.
                let remainder = self.config.capital.is_some()
                    && self.goods.batches.iter().any(|b| {
                        self.carried_by(id, b.holder)
                            && b.owner == Owner::Commons(0)
                            && b.material == Material::Food
                            && b.quantity > 0
                    });
                if !remainder {
                    self.agents[id].job = None;
                }
                self.log(
                    Some(id),
                    "Delivered",
                    format!("node={node} q={quantity} original_target={}", job.target),
                );
            }
            Intent::CancelJob => {
                self.cancel_action(id, "cancel job");
                self.cancel_job(id, "policy cancellation");
            }
            Intent::DiscardWaste => {
                let waste: Vec<_> = self
                    .goods
                    .batches
                    .iter()
                    .filter(|b| {
                        self.carried_by(id, b.holder)
                            && b.material == Material::Waste
                            && b.quantity > 0
                    })
                    .map(|b| (b.id, b.quantity, b.owner))
                    .collect();
                for (batch, q, owner) in waste {
                    self.goods.move_part(batch, q, owner, Holder::Loose(node));
                    self.log(Some(id), "WasteDropped", format!("batch={batch} q={q}"));
                }
            }
        }
        Ok(())
    }
    fn admit(&mut self) {
        for edge in 0..self.config.edges.len() {
            let occupied = self
                .agents
                .iter()
                .filter(|a| {
                    a.action == Action::Travel
                        && matches!(a.location,Location::Edge{edge:e,..}if e==edge)
                })
                .count();
            let slots = self.effective_edge(edge).capacity.saturating_sub(occupied);
            let mut queue: Vec<_> = self
                .agents
                .iter()
                .filter_map(|a| {
                    if let Action::Queued {
                        edge: e,
                        to,
                        joined,
                    } = a.action
                    {
                        (e == edge).then_some((joined, a.id, to))
                    } else {
                        None
                    }
                })
                .collect();
            queue.sort_unstable();
            for (joined, id, to) in queue.into_iter().take(slots) {
                // Independent instrumentation against the still-waiting eligible set.
                self.metrics.overtakes += self.agents.iter().filter(|a| matches!(a.action, Action::Queued {edge:e,joined:t,..} if e==edge && (t,a.id)<(joined,id))).count() as u64;
                let Location::Node(from) = self.agents[id].location else {
                    unreachable!()
                };
                self.agents[id].location = Location::Edge {
                    edge,
                    from,
                    to,
                    progress: 0,
                };
                self.capital_departure(id, from, to);
                self.infrastructure_departure(id, edge, from, to);
                self.path_departure(id, edge, from, to);
                self.agents[id].action = Action::Travel;
                self.sync_carts();
                self.log(
                    Some(id),
                    "Admitted",
                    format!("edge={edge} from={from} to={to}"),
                );
            }
        }
    }
    pub(super) fn next_time(&self, target: Time) -> Time {
        let mut next = target;
        for offer in &self.barter.offers {
            if offer.status == "OPEN" && offer.expires_at > self.now {
                next = next.min(offer.expires_at);
            }
        }
        if let Some((&t, _)) = self
            .transport_research
            .replay
            .range((self.now + 1)..)
            .next()
        {
            next = next.min(t);
        }
        if self.config.autonomous {
            next = next.min(self.next_policy);
        }
        for b in &self.goods.batches {
            if b.material == Material::Food && b.quantity > 0 && b.expires_at() > self.now {
                next = next.min(b.expires_at());
            }
        }
        for b in &self.boards {
            for c in &b.claims {
                if c.expires_at > self.now {
                    next = next.min(c.expires_at);
                }
            }
        }
        next = self.farm.next_boundary(self.now, next);
        next = self.paths.next_boundary(self.now, next);
        for a in &self.agents {
            if a.action == Action::Dead {
                continue;
            }
            let rate = performance(a.nutrition, a.fatigue);
            let threshold = if matches!(a.action, Action::Sleep { .. }) {
                if a.nutrition > 6 * HOUR {
                    a.nutrition - 6 * HOUR
                } else if a.nutrition > 0 {
                    a.nutrition
                } else {
                    Time::MAX / 4
                }
            } else {
                awake_threshold(a.nutrition, a.fatigue)
            };
            next = next.min(self.now.saturating_add(threshold));
            if a.nutrition == 0 {
                next = next.min(self.now + (24 * HOUR - a.starvation).max(1));
            }
            match a.action {
                Action::Travel => {
                    if let Location::Edge { edge, progress, .. } = a.location {
                        let left = self.traversal_nominal(a.id, edge) as u128 * SCALE - progress;
                        next = next.min(self.now + left.div_ceil(rate) as Time);
                    }
                }
                Action::BuildCrossing { project } => {
                    next = next.min(
                        self.now
                            + self.infrastructure.projects[project]
                                .remaining
                                .div_ceil(rate) as Time,
                    );
                }
                Action::BuildCart { project } => {
                    let remaining = self.capital.projects[project].remaining;
                    next = next.min(self.now + remaining.div_ceil(rate) as Time);
                }
                Action::Farm { plot } => {
                    let remaining = self.farm.plots[plot].crop.as_ref().unwrap().remaining;
                    next = next.min(self.now + remaining.div_ceil(rate) as Time);
                }
                Action::Gather { remaining, .. } => {
                    next = next.min(self.now + remaining.div_ceil(rate) as Time)
                }
                Action::Sleep { until } => next = next.min(until),
                _ => {}
            }
        }
        for h in [36, 48, 72] {
            let t = h * HOUR;
            if t > self.now {
                next = next.min(t);
            }
        }
        next.max(self.now + 1)
    }
    fn integrate(&mut self, to: Time) {
        let dt = to - self.now;
        self.integrate_market(dt);
        if let Some(mut journal) = self.settlement_research.take() {
            journal.integrate(self, dt);
            self.settlement_research = Some(journal);
        }
        let mut physiology_events = vec![];
        for b in &self.boards {
            if b.quantity > 0 {
                self.metrics.request_ms += dt as u128;
                self.metrics.request_quantum_ms += dt as u128 * b.quantity as u128;
            }
        }
        for id in 0..self.agents.len() {
            if self.agents[id].action == Action::Dead {
                continue;
            }
            let loaded = self.carrier_load(id) > 0;
            if self.config.farming.is_some() {
                let activity = match self.agents[id].action {
                    Action::Farm { plot } => {
                        if self.farm.plots[plot].crop.as_ref().unwrap().stage
                            == farming::Stage::Sowing
                        {
                            "sow"
                        } else {
                            "harvest"
                        }
                    }
                    Action::BuildCart { .. } => "cart_construction",
                    Action::BuildCrossing { .. } => "crossing_construction",
                    Action::Gather { .. } => "gather",
                    Action::Travel => "travel",
                    Action::Queued { .. } => "queue",
                    Action::Sleep { .. } => "sleep",
                    Action::Idle => "idle",
                    Action::Dead => unreachable!(),
                };
                *self.farm.activity_ms.entry(activity.into()).or_default() += dt as u128;
            }
            let traversal_work = if let Location::Edge { edge, .. } = self.agents[id].location {
                self.traversal_nominal(id, edge)
            } else {
                0
            };
            let a = &mut self.agents[id];
            let rate = performance(a.nutrition, a.fatigue);
            self.metrics.physiology_ms += dt as u128;
            let zero_time = (dt - a.nutrition).max(0);
            self.metrics.deficit_ms += zero_time as u128;
            if self.config.acceptance_audit {
                *self.acceptance.zero_ms.entry(id).or_default() += zero_time as u128;
            }
            if zero_time > 0 {
                a.starvation += zero_time;
            }
            let old_n = a.nutrition;
            let old_f = a.fatigue;
            a.nutrition = (a.nutrition - dt).max(0);
            a.fatigue = if matches!(a.action, Action::Sleep { .. }) {
                (a.fatigue - 2 * dt).max(0)
            } else {
                (a.fatigue + dt).min(24 * HOUR)
            };
            if (old_n > 6 * HOUR && a.nutrition <= 6 * HOUR)
                || (old_n > 0 && a.nutrition == 0)
                || (old_f < 22 * HOUR && a.fatigue >= 22 * HOUR)
            {
                self.wake.insert(id);
            }
            let new_rate = performance(a.nutrition, a.fatigue);
            if rate != new_rate && !matches!(a.action, Action::Sleep { .. }) {
                physiology_events.push((
                    id,
                    "PerformanceChanged",
                    format!(
                        "old_ppm={rate} new_ppm={new_rate} nutrition_ms={} fatigue_ms={}",
                        a.nutrition, a.fatigue
                    ),
                ));
            }
            if old_n > 0 && a.nutrition == 0 {
                physiology_events.push((
                    id,
                    "StarvationStarted",
                    format!("continuous_starvation_ms={}", a.starvation),
                ));
            }
            match &mut a.action {
                Action::Travel => {
                    if let Location::Edge { progress, .. } = &mut a.location {
                        let before = *progress;
                        *progress =
                            (*progress + rate * dt as u128).min(traversal_work as u128 * SCALE);
                        a.travel_since_food += *progress - before;
                    }
                }
                Action::BuildCrossing { project } => {
                    let p = &mut self.infrastructure.projects[*project];
                    p.remaining = p.remaining.saturating_sub(rate * dt as u128);
                    *p.labor_ms.entry(id).or_default() += dt as u128;
                }
                Action::BuildCart { project } => {
                    self.capital.projects[*project].remaining = self.capital.projects[*project]
                        .remaining
                        .saturating_sub(rate * dt as u128);
                    self.capital.projects[*project].labor_ms += dt as u128;
                }
                Action::Farm { plot } => {
                    let crop = self.farm.plots[*plot].crop.as_mut().unwrap();
                    crop.remaining = crop.remaining.saturating_sub(rate * dt as u128);
                    self.farm.labor_ms += dt as u128;
                }
                Action::Gather { remaining, .. } => {
                    *remaining = remaining.saturating_sub(rate * dt as u128)
                }
                Action::Sleep { .. } => self.metrics.sleep_ms += dt as u128,
                Action::Queued { .. } => {
                    self.metrics.queue_ms += dt as u128;
                    if loaded {
                        self.metrics.loaded_queue_ms += dt as u128;
                    } else {
                        self.metrics.unloaded_queue_ms += dt as u128;
                    }
                }
                Action::Idle => self.metrics.idle_ms += dt as u128,
                Action::Dead => {}
            }
        }
        if self.config.infrastructure.is_some() {
            for a in &self.agents {
                match a.action {
                    Action::Queued { edge, .. } => {
                        *self.infrastructure.queue_ms.entry(edge).or_default() += dt as u128
                    }
                    Action::Travel => {
                        if let Location::Edge { edge, .. } = a.location {
                            *self.infrastructure.occupancy_ms.entry(edge).or_default() +=
                                dt as u128;
                        }
                    }
                    _ => {}
                }
            }
        }
        self.integrate_capital(dt);
        self.integrate_paths(dt);
        self.now = to;
        self.sync_carts();
        for (id, kind, detail) in physiology_events {
            self.log(Some(id), kind, detail);
        }
    }
    fn boundary(&mut self) {
        // Recovery precedes simultaneous arrivals; a completed passage then renews use.
        self.recover_paths();
        // Arrival first determines exact location at a coincident death; no decision yet.
        for id in 0..self.agents.len() {
            if self.agents[id].action == Action::Travel
                && let Location::Edge {
                    edge, to, progress, ..
                } = self.agents[id].location
                && progress >= self.traversal_nominal(id, edge) as u128 * SCALE
            {
                self.agents[id].location = Location::Node(to);
                self.sync_carts();
                self.capital_arrival(id);
                self.infrastructure_arrival(id);
                self.path_arrival(id);
                self.agents[id].action = Action::Idle;
                self.wake.insert(id);
                self.log(Some(id), "Arrived", format!("node={to} edge={edge}"));
            }
        }
        let expired: Vec<_> = self
            .goods
            .batches
            .iter()
            .filter(|b| {
                b.quantity > 0 && b.material == Material::Food && b.expires_at() <= self.now
            })
            .map(|b| b.id)
            .collect();
        for batch in expired {
            let b = self.goods.batches[batch].clone();
            self.goods.batches[batch].material = Material::Waste;
            self.metrics.spoiled += b.quantity;
            if matches!(b.holder, Holder::Commons(_)) {
                self.metrics.spoiled_commons += b.quantity;
            }
            let carrier = match b.holder {
                Holder::Agent(id) => Some(id),
                Holder::Cart(c) => self.capital.carts[c].user,
                _ => None,
            };
            if let Some(id) = carrier
                && matches!(
                    self.agents[id].action,
                    Action::Travel | Action::Queued { .. }
                )
            {
                self.metrics.spoiled_transit += b.quantity;
                if b.owner == Owner::Commons(0) {
                    self.diagnostics.cargo_spoiled(b.quantity);
                }
            }
            self.log(
                None,
                "Spoiled",
                format!(
                    "batch={batch} q={} holder={:?} created={} deadline={}",
                    b.quantity,
                    b.holder,
                    b.created_at,
                    b.expires_at()
                ),
            );
            for id in 0..self.agents.len() {
                if matches!(self.agents[id].action,Action::Gather{batch:b,..}if b==batch) {
                    self.cancel_action(id, "reserved food spoiled");
                    self.wake.insert(id);
                }
            }
        }
        for id in 0..self.agents.len() {
            if self.agents[id].action != Action::Dead && self.agents[id].starvation >= 24 * HOUR {
                let holder = match self.agents[id].location {
                    Location::Node(n) => Holder::Loose(n),
                    Location::Edge {
                        edge,
                        from,
                        progress,
                        ..
                    } => Holder::Edge {
                        edge,
                        from,
                        progress,
                    },
                };
                let cargo: Vec<_> = self
                    .goods
                    .batches
                    .iter()
                    .filter(|b| b.holder == Holder::Agent(id) && b.quantity > 0)
                    .map(|b| (b.id, b.quantity, b.owner, b.material))
                    .collect();
                for (batch, q, owner, material) in cargo {
                    if material == Material::Food {
                        self.metrics.carried_at_death += q;
                        if matches!(holder, Holder::Edge { .. }) {
                            self.metrics.stranded += q;
                        }
                    }
                    self.goods.move_part(batch, q, owner, holder);
                }
                self.capital_death(id);
                self.infrastructure.active_trips.remove(&id);
                self.paths.active.remove(&id);
                self.gather_communal.remove(&id);
                self.cancel_job(id, "starvation death");
                self.agents[id].action = Action::Dead;
                self.agents[id].death_time = Some(self.now);
                self.metrics.deaths += 1;
                self.log(
                    Some(id),
                    "Died",
                    format!(
                        "cause=starvation nutrition_ms=0 starvation_ms={} location={:?}",
                        self.agents[id].starvation, self.agents[id].location
                    ),
                );
            }
        }
        self.finish_farming();
        self.finish_carts();
        self.finish_crossings();
        for id in 0..self.agents.len() {
            match self.agents[id].action.clone() {
                Action::Gather {
                    batch,
                    remaining: 0,
                } => {
                    let communal = self.gather_communal.remove(&id).unwrap_or(false);
                    let owner = if communal {
                        Owner::Commons(0)
                    } else {
                        Owner::Agent(id)
                    };
                    self.load_carrier(id, batch, 1, owner);
                    self.agents[id].action = Action::Idle;
                    if communal && let Some(j) = &mut self.agents[id].job {
                        j.acquired += 1;
                    }
                    let food = self.goods.batches[batch].material == Material::Food;
                    if food {
                        self.metrics.gathered += 1;
                    }
                    self.wake.insert(id);
                    self.log(
                        Some(id),
                        if food { "Gathered" } else { "WoodGathered" },
                        format!("source_batch={batch} q=1 owner={owner:?}"),
                    );
                }
                Action::Sleep { until } if until <= self.now => {
                    if !self.config.autonomous {
                        self.agents[id].action = Action::Idle;
                    }
                    self.wake.insert(id);
                }
                _ => {}
            }
        }
        for index in 0..self.boards.len() {
            let old = self.boards[index].claims.len();
            for c in &self.boards[index].claims {
                if c.expires_at <= self.now {
                    self.diagnostics
                        .claim_expired(c.agent, self.boards[index].node, c.expires_at);
                }
            }
            self.boards[index]
                .claims
                .retain(|c| c.expires_at > self.now);
            let count = old - self.boards[index].claims.len();
            if count > 0 {
                self.metrics.lease_expiries += count as u64;
                self.log(
                    None,
                    "LeaseExpired",
                    format!(
                        "board={} count={count}; workers not notified",
                        self.boards[index].node
                    ),
                );
            }
        }
        self.expire_barter();
        if self.config.autonomous {
            if self.now >= self.next_policy {
                self.wake.extend(0..self.agents.len());
                self.next_policy = self.now + self.config.policy_ms;
            }
            self.decisions();
        }
        self.admit();
        if [36 * HOUR, 48 * HOUR, 72 * HOUR].contains(&self.now) {
            self.metrics.survivors.insert(
                self.now,
                self.agents
                    .iter()
                    .filter(|a| a.action != Action::Dead)
                    .count(),
            );
        }
    }
    fn refresh_boards(&mut self, observed: &BTreeSet<usize>) {
        for index in 0..self.boards.len() {
            let node = self.boards[index].node;
            if !observed.contains(&node) {
                continue;
            }
            let need: u64 = self
                .agents
                .iter()
                .filter(|a| {
                    a.action != Action::Dead
                        && a.location == Location::Node(node)
                        && a.nutrition < 8 * HOUR
                })
                .map(|a| {
                    let personal: u64 = self
                        .goods
                        .batches
                        .iter()
                        .filter(|b| {
                            self.carried_by(a.id, b.holder)
                                && b.owner == Owner::Agent(a.id)
                                && b.material == Material::Food
                                && b.expires_at() > self.now
                        })
                        .map(|b| b.quantity)
                        .sum();
                    ((12 * HOUR - a.nutrition) as u64)
                        .div_ceil(HOUR as u64)
                        .saturating_sub(personal)
                })
                .sum();
            let accessible: u64 = self
                .goods
                .batches
                .iter()
                .filter(|b| {
                    b.material == Material::Food
                        && b.expires_at() > self.now
                        && (matches!(b.holder,Holder::Commons(n)|Holder::Loose(n)if n==node))
                        && b.owner == Owner::Commons(0)
                })
                .map(|b| b.quantity)
                .sum();
            let quantity =
                need.saturating_sub(accessible)
                    .max(if self.config.boards.first() == Some(&node) {
                        self.request_backlog
                    } else {
                        0
                    });
            let changed = self.boards[index].quantity != quantity;
            self.boards[index].quantity = quantity;
            self.boards[index].refreshed_at = self.now;
            if changed {
                self.log(
                    None,
                    "RequestObserved",
                    format!("node={node} local_demand={quantity}"),
                );
            }
        }
    }
    fn decisions(&mut self) {
        let mut ids: Vec<_> = std::mem::take(&mut self.wake)
            .into_iter()
            .filter(|&id| !matches!(self.agents[id].action, Action::Dead | Action::Travel))
            .filter(|id| {
                self.transport_research
                    .actors
                    .as_ref()
                    .is_none_or(|ids| ids.contains(id))
            })
            .collect();
        let observed: BTreeSet<_> = ids
            .iter()
            .filter_map(|&id| {
                if let Location::Node(n) = self.agents[id].location {
                    Some(n)
                } else {
                    None
                }
            })
            .collect();
        self.refresh_boards(&observed);
        if self.config.reverse_evaluation {
            ids.reverse();
        }
        let mut decisions = vec![];
        let mut stale_events = vec![];
        for id in ids {
            self.observe_farming(id);
            self.observe_capital(id);
            self.observe_infrastructure(id);
            let o = self.observe(id).unwrap();
            if let Some(local) = &o.local {
                if self.agents[id].travel_since_food > 0
                    && local.food.iter().any(|f| {
                        f.quantity > 0
                            && (f.owner == Owner::None
                                || f.owner == Owner::Agent(id)
                                || f.owner == Owner::Commons(0))
                    })
                {
                    self.metrics.food_trips += 1;
                    self.metrics.food_travel_mm += self.agents[id].travel_since_food / SCALE;
                    self.agents[id].travel_since_food = 0;
                }
                if self.agents[id].goal == Some((local.node, true)) {
                    let expected = self.agents[id]
                        .memory
                        .nodes
                        .get(&local.node)
                        .is_some_and(|v| v.food.iter().any(|f| f.quantity > 0));
                    let found = local.food.iter().any(|f| {
                        f.quantity > 0
                            && (f.owner == Owner::None
                                || f.owner == Owner::Agent(id)
                                || f.owner == Owner::Commons(0))
                    });
                    if expected && !found {
                        self.metrics.stale_failures += 1;
                        stale_events.push((id, local.node));
                    }
                    self.agents[id].goal = None;
                }
                self.agents[id]
                    .memory
                    .nodes
                    .insert(local.node, local.clone());
            }
            let rng_before = self.agents[id].rng;
            let a = &mut self.agents[id];
            if self.transport_research.audit_forecasts {
                let (d, audit) = policy::audit_meals(
                    &o,
                    &a.memory,
                    &mut a.rng.clone(),
                    self.config.persistence == Some(persistence::Arm::ForecastCorrected),
                );
                self.acceptance.record(&o, &d, audit);
            }
            let decision = if let Some(config) = &self.config.infrastructure {
                policy::decide_infrastructure(
                    &o,
                    &a.memory,
                    &mut a.rng,
                    self.config
                        .farming
                        .as_ref()
                        .map(|f| (&self.farm.memories[id], &f.rules)),
                    self.config
                        .capital
                        .as_ref()
                        .map(|c| (&self.capital.memories[id], &c.rules)),
                    &self.infrastructure.memories[id],
                    config.enabled,
                )
            } else if let Some(config) = &self.config.capital {
                policy::decide_capital(
                    &o,
                    &a.memory,
                    &mut a.rng,
                    self.config
                        .farming
                        .as_ref()
                        .map(|f| (&self.farm.memories[id], &f.rules)),
                    &self.capital.memories[id],
                    &config.rules,
                )
            } else if let Some(config) = &self.config.farming {
                policy::decide_farming(
                    &o,
                    &a.memory,
                    &mut a.rng,
                    &self.farm.memories[id],
                    &config.rules,
                )
            } else if let Some(arm) = self.config.persistence {
                let state = self.persistence.states.entry(id).or_default();
                let d = policy::decide_persistent(&o, &a.memory, &mut a.rng, arm, state);
                self.persistence.collect_decision(id);
                d
            } else if self.config.acceptance_audit {
                let (d, audit) = policy::decide_audited(
                    &o,
                    &a.memory,
                    &mut a.rng,
                    self.config.diagnostic_policy,
                );
                self.acceptance.record(&o, &d, audit);
                d
            } else {
                policy::decide_variant(&o, &a.memory, &mut a.rng, self.config.diagnostic_policy)
            };
            let decision = self.barter_decision(id, &o, decision);
            self.record_capital_evaluation(id, &o);
            let a = &self.agents[id];
            self.diagnostics.observe(&o, &a.memory, &decision);
            self.meals.observe(
                &o,
                &a.memory,
                &decision,
                rng_before,
                self.config.diagnostic_policy,
            );
            if self.coherence.enabled {
                let mut journal = std::mem::take(&mut self.coherence);
                journal.observe(self, &o, &decision, rng_before);
                self.coherence = journal;
            }
            if self.shadow.enabled {
                let mut journal = std::mem::take(&mut self.shadow);
                journal.observe(self, &o, &decision, rng_before);
                self.shadow = journal;
            }
            if let (Action::Queued { edge, .. }, Some((target, true))) = (&o.action, o.goal) {
                let leaving = match decision.intent {
                    Intent::Travel { edge: next, .. } => next != *edge,
                    Intent::Eat { .. } | Intent::Sleep | Intent::CancelJob => true,
                    _ => false,
                };
                if leaving && policy::goal_is_fatal(&o, &self.agents[id].memory, target) {
                    self.metrics.infeasible_abandons += 1;
                }
            }
            decisions.push((id, decision, o.nutrition, o.starvation));
        }
        stale_events.sort_unstable();
        for (id, node) in stale_events {
            self.log(
                Some(id),
                "StaleFoodTrip",
                format!("node={node} remembered food absent/inaccessible"),
            );
        }
        if let Some(commands) = self.transport_research.replay.remove(&self.now) {
            for (id, intent) in commands {
                decisions.push((
                    id,
                    Decision {
                        intent,
                        reason: "research recorded background command".into(),
                    },
                    self.agents[id].nutrition,
                    self.agents[id].starvation,
                ));
            }
        }
        decisions.sort_by_key(|(id, _, _, _)| *id);
        for (id, d, n, s) in decisions {
            if self.transport_research.record {
                self.transport_research
                    .tape
                    .push((self.now, id, d.intent.clone()));
            }
            self.log(
                Some(id),
                "Decision",
                format!(
                    "intent={:?} nutrition_ms={n} starvation_ms={s} reason={}",
                    d.intent, d.reason
                ),
            );
            if d.intent == Intent::CancelJob {
                self.cancel_action(id, "cancel job");
                self.cancel_job(id, &d.reason);
                continue;
            }
            if let Err(err) = self.execute(id, d.intent) {
                self.log(Some(id), "Rejected", err);
            }
        }
    }
    pub fn advance_to(&mut self, target: Time) -> Result<(), String> {
        if target < self.now {
            return Err("time reversal".into());
        }
        if !self.initialized {
            self.initialized = true;
            self.boundary();
            self.audit()?;
        }
        while self.now < target {
            let next = self.next_time(target);
            self.integrate(next);
            self.boundary();
            self.audit()?;
        }
        Ok(())
    }
}
