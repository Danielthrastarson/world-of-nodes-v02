//! Institutional kernel endpoints. Remote state is never a command-validation oracle.
use super::*;

impl World {
    pub fn command_memory(&self, id: usize) -> Option<&Memory> {
        self.commands.memories.get(id)
    }
    pub fn command_journal(&self) -> &Journal {
        &self.commands.journal
    }
    fn command_actor_node(&self, id: usize) -> Result<usize, String> {
        if !self.config.commands {
            return Err("office command institution disabled".into());
        }
        self.information_actor_node(id)
    }
    pub(crate) fn commands_offered(&mut self, office: authority::Office) {
        if !self.config.commands {
            return;
        }
        self.commands.memories[office.holder]
            .offered_offices
            .insert(office.id, office.clone());
        self.commands.journal.add("offices_offered");
        self.log(
            Some(office.holder),
            "OfficeOffered",
            format!(
                "office={}; issuer={}; acceptance and duty remain voluntary",
                office.id, office.appointed_by
            ),
        );
    }
    /// Explicit local acceptance, not unilateral appointment-as-obedience.
    pub fn accept_office(&mut self, actor: usize, office: usize) -> Result<(), String> {
        let node = self.command_actor_node(actor)?;
        if self.is_child(actor) {
            return Err("adult office capability required".into());
        }
        let offered = self.commands.memories[actor]
            .offered_offices
            .get(&office)
            .ok_or("no personally received office offer")?
            .clone();
        if offered.holder != actor
            || self.command_actor_node(offered.appointed_by)? != node
            || self.is_child(offered.appointed_by)
        {
            return Err("living adult appointing issuer must be colocated for acceptance".into());
        }
        let actual = self
            .authority
            .offices
            .get_mut(office)
            .ok_or("office no longer exists")?;
        if actual != &offered || actual.status != authority::OfficeStatus::Offered {
            return Err("office offer no longer valid".into());
        }
        actual.status = authority::OfficeStatus::Appointed;
        let certificate = actual.clone();
        self.commands.memories[actor]
            .offered_offices
            .remove(&office);
        self.commands.memories[actor]
            .accepted_offices
            .insert(office, certificate.clone());
        self.commands.memories[offered.appointed_by]
            .accepted_offices
            .insert(office, certificate);
        self.commands.journal.add("offices_accepted");
        self.cancel_action(actor, "voluntary local office acceptance");
        self.log(Some(actor), "OfficeAccepted", format!("office={office}; issuer={}; scope=local public crossing commission; no private labor entitlement", offered.appointed_by));
        Ok(())
    }
    /// Observe descriptions of local potential projects, without surveying remote work.
    pub fn observe_command_sites(&mut self, actor: usize) -> Result<(), String> {
        let node = self.command_actor_node(actor)?;
        for p in self
            .infrastructure
            .projects
            .iter()
            .filter(|p| p.site.node == node)
        {
            self.commands.memories[actor].known_sites.insert(
                p.id,
                KnownSite {
                    project: p.id,
                    settlement: node,
                    observed_at: self.now,
                },
            );
        }
        Ok(())
    }
    /// Issuance creates only an envelope in the issuer's own memory. A known office
    /// certificate is deliberately stale if its distant holder has since died.
    pub fn issue_order(
        &mut self,
        issuer: usize,
        office: usize,
        objective: Objective,
        expires_at: Time,
    ) -> Result<CommandId, String> {
        let node = self.command_actor_node(issuer)?;
        if self.is_child(issuer) {
            return Err("adult issuer capability required".into());
        }
        self.observe_command_sites(issuer)?;
        let certificate = self.commands.memories[issuer]
            .accepted_offices
            .get(&office)
            .ok_or("issuer has no personally held accepted office certificate")?
            .clone();
        if certificate.appointed_by != issuer
            || certificate.status != authority::OfficeStatus::Appointed
        {
            return Err("issuer lacks accepted authority over office".into());
        }
        if expires_at <= self.now {
            return Err("order must have a future explicit deadline".into());
        }
        if let Objective::CommissionCrossing {
            project,
            settlement,
        } = &objective
            && self.commands.memories[issuer]
                .known_sites
                .get(project)
                .is_none_or(|s| s.settlement != *settlement)
        {
            return Err("project location not personally known to issuer".into());
        }
        let id = self.commands.next_message;
        self.commands.next_message += 1;
        let order = Order {
            id,
            issuer,
            polity: certificate.polity,
            recipient: certificate.holder,
            office,
            objective,
            issued_at: self.now,
            expires_at,
        };
        self.commands.memories[issuer].envelopes.insert(
            id,
            Receipt {
                envelope: Envelope::Order(order.clone()),
                sender: issuer,
                received_at: self.now,
                node,
            },
        );
        self.commands.journal.add("orders_issued");
        self.log(
            Some(issuer),
            "OrderIssued",
            format!("{order:?}; no recipient state changed"),
        );
        Ok(id)
    }
    pub fn issue_revocation(
        &mut self,
        issuer: usize,
        command: CommandId,
    ) -> Result<MessageId, String> {
        let node = self.command_actor_node(issuer)?;
        let Some(Receipt {
            envelope: Envelope::Order(order),
            ..
        }) = self.commands.memories[issuer].envelopes.get(&command)
        else {
            return Err("issuer does not hold original order".into());
        };
        if order.issuer != issuer {
            return Err("only original authorized issuer may revoke".into());
        }
        let r = Revocation {
            id: self.commands.next_message,
            issuer,
            recipient: order.recipient,
            office: order.office,
            command,
            issued_at: self.now,
        };
        self.commands.next_message += 1;
        self.commands.memories[issuer].envelopes.insert(
            r.id,
            Receipt {
                envelope: Envelope::Revoke(r.clone()),
                sender: issuer,
                received_at: self.now,
                node,
            },
        );
        self.commands.journal.add("revocations_sent");
        self.log(
            Some(issuer),
            "RevocationIssued",
            format!("{r:?}; recipient continues until actual receipt"),
        );
        Ok(r.id)
    }
    pub fn receive_command(
        &mut self,
        sender: usize,
        recipient: usize,
        message: MessageId,
    ) -> Result<(), String> {
        let node = self.command_actor_node(sender)?;
        if sender == recipient || self.command_actor_node(recipient)? != node {
            return Err("distinct living participants must be physically colocated".into());
        }
        let envelope = self.commands.memories[sender]
            .envelopes
            .get(&message)
            .ok_or("sender does not hold envelope")?
            .envelope
            .clone();
        if self.commands.memories[recipient]
            .envelopes
            .contains_key(&message)
        {
            self.commands.journal.add("duplicate_transmissions");
            self.complete_command_delivery(sender, message, recipient);
            return Ok(());
        }
        self.commands.memories[recipient].envelopes.insert(
            message,
            Receipt {
                envelope: envelope.clone(),
                sender,
                received_at: self.now,
                node,
            },
        );
        self.commands.journal.add("messages_transmitted");
        self.commands.journal.add(match envelope {
            Envelope::Order(_) => "orders_transmitted",
            Envelope::Revoke(_) => "revocations_transmitted",
        });
        let t = Transmission {
            message,
            sender,
            recipient,
            node,
            issued_at: envelope.issued_at(),
            received_at: self.now,
        };
        self.commands.journal.transmissions.push(t.clone());
        self.log(
            Some(recipient),
            "CommandReceived",
            format!("{t:?}; payload={envelope:?}"),
        );
        if envelope.recipient() == recipient {
            match envelope {
                Envelope::Order(o) => self.validate_received_order(recipient, o),
                Envelope::Revoke(r) => self.validate_received_revocation(recipient, r),
            }
        }
        self.cancel_action(sender, "local command handoff");
        self.complete_command_delivery(sender, message, recipient);
        // No wake: newly received obligations compete at the normal policy boundary.
        Ok(())
    }
    fn invalid_command(&mut self, recipient: usize, message: MessageId, why: &str) {
        self.commands.memories[recipient]
            .invalid
            .insert(message, why.into());
        self.commands.journal.add("orders_invalid");
        self.log(
            Some(recipient),
            "OrderInvalid",
            format!("message={message}; reason={why}"),
        );
    }
    fn validate_received_order(&mut self, recipient: usize, order: Order) {
        self.commands.journal.add("orders_received");
        let m = &self.commands.memories[recipient];
        let Some(cert) = m.accepted_offices.get(&order.office) else {
            self.invalid_command(recipient, order.id, "NO_ACCEPTED_OFFICE");
            return;
        };
        if cert.holder != recipient
            || cert.appointed_by != order.issuer
            || cert.polity != order.polity
        {
            self.invalid_command(recipient, order.id, "UNAUTHORIZED_ISSUER_OR_OFFICE");
            return;
        }
        if !matches!(order.objective, Objective::CommissionCrossing { settlement, .. } if settlement == cert.settlement)
        {
            self.invalid_command(recipient, order.id, "OUT_OF_SCOPE");
            return;
        }
        let (status, reason) = if m.received_revocations.contains(&order.id) {
            (Status::Revoked, "REVOCATION_ALREADY_RECEIVED")
        } else if order.expires_at <= self.now {
            (Status::Expired, "EXPIRED_AT_RECEIPT")
        } else {
            (Status::Received, "VALID_SCOPED_OFFICE_ORDER")
        };
        let obligation = RoleObligation {
            command: order.id,
            source_issuer: order.issuer,
            recipient_office: order.office,
            objective: order.objective,
            issued_at: order.issued_at,
            received_at: self.now,
            expires_at: order.expires_at,
            status,
            reason: reason.into(),
            changed_at: self.now,
        };
        self.commands.memories[recipient]
            .obligations
            .insert(order.id, obligation);
        if status == Status::Received {
            self.commands.journal.add("orders_valid");
            self.commands.journal.add("obligations_formed");
        } else {
            self.commands.journal.add(if status == Status::Expired {
                "orders_expired_at_receipt"
            } else {
                "orders_revoked_at_receipt"
            });
        }
        self.record_command_transition(recipient, order.id, status, reason);
    }
    fn validate_received_revocation(&mut self, recipient: usize, r: Revocation) {
        let m = &self.commands.memories[recipient];
        if m.accepted_offices
            .get(&r.office)
            .is_none_or(|o| o.holder != recipient || o.appointed_by != r.issuer)
        {
            self.invalid_command(recipient, r.id, "REVOCATION_UNAUTHORIZED");
            return;
        }
        self.commands.memories[recipient]
            .received_revocations
            .insert(r.command);
        self.commands.journal.add("revocations_received");
        let completed = self.commands.memories[recipient]
            .obligations
            .get(&r.command)
            .is_some_and(|o| o.status == Status::Completed);
        if self.commands.memories[recipient]
            .obligations
            .get(&r.command)
            .is_some_and(|o| !o.status.terminal())
        {
            self.set_command_status(
                recipient,
                r.command,
                Status::Revoked,
                "REVOCATION_PHYSICALLY_RECEIVED",
            );
        }
        self.log(Some(recipient), "RevocationReceived", format!("message={}; command={}; completed={completed}; no retroactive undo of lawful commission or physical work", r.id, r.command));
    }
    fn record_command_transition(
        &mut self,
        person: usize,
        command: CommandId,
        status: Status,
        reason: &str,
    ) {
        self.commands.journal.transitions.push(Transition {
            person,
            command,
            at: self.now,
            status,
            reason: reason.into(),
        });
        self.commands.journal.add(match status {
            Status::Received => "obligations_received",
            Status::Active => "obligations_active",
            Status::Paused => "obligations_paused",
            Status::Completed => "obligations_completed",
            Status::Failed => "obligations_failed",
            Status::Expired => "obligations_expired",
            Status::Refused => "obligations_refused",
            Status::Revoked => "obligations_revoked",
        });
        self.log(
            Some(person),
            "RoleObligation",
            format!("command={command}; status={status:?}; reason={reason}"),
        );
    }
    fn set_command_status(
        &mut self,
        actor: usize,
        command: CommandId,
        status: Status,
        reason: &str,
    ) {
        let Some(o) = self.commands.memories[actor].obligations.get_mut(&command) else {
            return;
        };
        if o.status == status && o.reason == reason {
            return;
        }
        o.status = status;
        o.reason = reason.into();
        o.changed_at = self.now;
        let target = match o.objective {
            Objective::CommissionCrossing { settlement, .. } => Some(settlement),
            _ => None,
        };
        if status.terminal() && self.commands.memories[actor].pursuing == Some(command) {
            self.commands.memories[actor].pursuing = None;
            if target.is_some_and(|n| self.agents[actor].goal == Some((n, false))) {
                self.agents[actor].goal = None;
                if matches!(self.agents[actor].action, Action::Queued { .. }) {
                    self.cancel_action(actor, "role undertaking ended at local decision point");
                }
            }
        }
        if status == Status::Paused
            && self.commands.memories[actor].pursuing == Some(command)
            && target.is_some_and(|n| self.agents[actor].goal == Some((n, false)))
            && matches!(self.agents[actor].action, Action::Queued { .. })
        {
            self.agents[actor].goal = None;
            self.cancel_action(actor, "role route queue interrupted; obligation retained");
        }
        self.record_command_transition(actor, command, status, reason);
    }
    pub(crate) fn record_command_choice(&mut self, actor: usize, d: &Decision) {
        if !self.config.commands {
            return;
        }
        let command = self.observe(actor).and_then(|o| {
            selected_role_command(
                &o,
                &self.agents[actor].memory,
                &self.commands.memories[actor],
                d,
            )
        });
        if let Some(command) = command {
            self.commands.memories[actor].pursuing = Some(command);
            self.set_command_status(
                actor,
                command,
                Status::Active,
                "PERSON_SELECTED_ROLE_UNDERTAKING",
            );
        }
    }
    /// A controlled explicit choice. It is not an automatic refusal policy or a punishment.
    pub fn refuse_order(
        &mut self,
        actor: usize,
        command: CommandId,
        reason: &str,
    ) -> Result<(), String> {
        self.command_actor_node(actor)?;
        self.command_before_decision(actor);
        let o = self.commands.memories[actor]
            .obligations
            .get(&command)
            .ok_or("no understood valid role obligation")?;
        if o.status.terminal() {
            return Err("role obligation no longer active".into());
        }
        if reason.is_empty() {
            return Err("explicit choice reason required".into());
        }
        self.set_command_status(
            actor,
            command,
            Status::Refused,
            &format!("CONTROLLED_EXPLICIT_CHOICE: {reason}"),
        );
        Ok(())
    }
    /// Attempt the existing institutional operation at the actual locality.
    /// Completion is commission authorization, not materials, worker obedience or a bridge.
    pub fn fulfil_order(&mut self, actor: usize, command: CommandId) -> Result<(), String> {
        let node = self.command_actor_node(actor)?;
        self.command_before_decision(actor);
        let o = self.commands.memories[actor]
            .obligations
            .get(&command)
            .ok_or("no valid received obligation")?
            .clone();
        if o.status.terminal() {
            return Err("obligation terminal".into());
        }
        let needs = self.agents[actor].needs();
        if needs.critical_hunger() || needs.critical_fatigue() {
            return Err("critical survival defers role obligation".into());
        }
        let Objective::CommissionCrossing {
            project,
            settlement,
        } = o.objective
        else {
            return Err("unsupported obligation".into());
        };
        if node != settlement {
            return Err("local institutional action requires assigned settlement".into());
        }
        self.set_command_status(actor, command, Status::Active, "ATTEMPT_LOCAL_COMMISSION");
        // These kernel checks are exclusively local at this point, not remote planning knowledge.
        if self
            .observe_authority(actor)
            .is_some_and(|v| v.posted_commissions.iter().any(|c| c.project == project))
        {
            self.set_command_status(
                actor,
                command,
                Status::Completed,
                "LOCAL_OBJECTIVE_ALREADY_AUTHORIZED",
            );
            return Ok(());
        }
        match self.commission_crossing(actor, o.recipient_office, project) {
            Ok(()) => {
                self.set_command_status(
                    actor,
                    command,
                    Status::Completed,
                    "LOCAL_COMMISSION_AUTHORIZED_NOT_PHYSICAL_SUCCESS",
                );
                Ok(())
            }
            Err(e) => {
                self.set_command_status(
                    actor,
                    command,
                    Status::Failed,
                    &format!("LOCAL_INSTITUTIONAL_FAILURE: {e}"),
                );
                Err(e)
            }
        }
    }
    pub(crate) fn command_before_decision(&mut self, actor: usize) {
        if !self.config.commands || actor >= self.commands.memories.len() {
            return;
        }
        if self.agents[actor].action == Action::Dead {
            self.command_person_died(actor);
            return;
        }
        let _ = self.observe_command_sites(actor);
        let Some(o) = self.observe(actor) else {
            return;
        };
        let pending: Vec<_> = self.commands.memories[actor]
            .obligations
            .values()
            .filter(|x| !x.status.terminal())
            .cloned()
            .collect();
        for obligation in pending {
            let reason = if obligation.expires_at <= self.now {
                self.set_command_status(
                    actor,
                    obligation.command,
                    Status::Expired,
                    "KNOWN_DEADLINE_REACHED",
                );
                continue;
            } else if o.body_factor == 0 {
                Some("INCAPACITATED")
            } else if person::Needs::from_observation(&o).critical_hunger() {
                Some("CRITICAL_HUNGER")
            } else if person::Needs::from_observation(&o).critical_fatigue() {
                Some("CRITICAL_FATIGUE")
            } else if let Objective::CommissionCrossing { settlement, .. } = obligation.objective {
                if matches!(o.location, Location::Node(n) if n != settlement) {
                    match policy::coherent::route(&o, &self.agents[actor].memory, settlement) {
                        None => Some("NO_KNOWN_ROUTE"),
                        Some(r) if r.starvation >= 24 * HOUR => Some("PREDICTED_DEATH"),
                        _ => None,
                    }
                } else {
                    None
                }
            } else {
                Some("UNSUPPORTED_SCOPE")
            };
            if let Some(why) = reason {
                self.set_command_status(actor, obligation.command, Status::Paused, why);
            } else if obligation.status == Status::Paused {
                self.set_command_status(
                    actor,
                    obligation.command,
                    Status::Received,
                    "PAUSE_CONDITION_RESOLVED",
                );
            }
        }
    }
    /// A personally known deadline is a real event, not contingent on a caller's
    /// checkpoint or the next optional deliberation tick. Unknown messages do not
    /// participate because they have not created a personal obligation.
    pub(crate) fn command_next_boundary(&self, now: Time, next: Time) -> Time {
        if !self.config.commands {
            return next;
        }
        self.commands
            .memories
            .iter()
            .flat_map(|m| m.obligations.values())
            .filter(|o| !o.status.terminal() && o.expires_at > now)
            .map(|o| o.expires_at)
            .min()
            .map_or(next, |deadline| next.min(deadline))
    }
    /// Called only from the kernel boundary before local decisions/admission. It
    /// never withdraws an in-progress traversal, and does not cancel another goal.
    pub(crate) fn finish_commands(&mut self) {
        if !self.config.commands {
            return;
        }
        let expired: Vec<_> = self
            .commands
            .memories
            .iter()
            .enumerate()
            .flat_map(|(actor, m)| {
                m.obligations
                    .values()
                    .filter(|o| !o.status.terminal() && o.expires_at <= self.now)
                    .map(move |o| (actor, o.command))
            })
            .collect();
        for (actor, command) in expired {
            self.set_command_status(actor, command, Status::Expired, "KNOWN_DEADLINE_REACHED");
            self.wake.insert(actor);
        }
    }
    pub(crate) fn command_person_died(&mut self, actor: usize) {
        if !self.config.commands || actor >= self.commands.memories.len() {
            return;
        }
        let pending: Vec<_> = self.commands.memories[actor]
            .obligations
            .values()
            .filter(|o| !o.status.terminal())
            .map(|o| o.command)
            .collect();
        for command in pending {
            self.set_command_status(actor, command, Status::Failed, "PERSON_DIED");
        }
        self.cancel_command_delivery(actor, "PERSON_DIED");
    }
    pub fn begin_command_delivery(
        &mut self,
        actor: usize,
        message: MessageId,
        recipient: usize,
        rendezvous: usize,
    ) -> Result<(), String> {
        self.command_actor_node(actor)?;
        if actor == recipient || recipient >= self.agents.len() || rendezvous >= self.config.nodes {
            return Err("invalid fixed delivery objective".into());
        }
        let m = &mut self.commands.memories[actor];
        if m.delivery.is_some() || !m.envelopes.contains_key(&message) {
            return Err("already delivering or envelope not held".into());
        }
        m.delivery = Some(Delivery {
            message,
            recipient,
            rendezvous,
            created_at: self.now,
        });
        self.commands.journal.add("deliveries_started");
        self.log(Some(actor), "CommandDeliveryStarted", format!("message={message}; recipient={recipient}; rendezvous={rendezvous}; controlled voluntarily accepted purpose; no remote tracking"));
        Ok(())
    }
    fn complete_command_delivery(&mut self, actor: usize, message: MessageId, recipient: usize) {
        if self.commands.memories[actor]
            .delivery
            .as_ref()
            .is_some_and(|u| u.message == message && u.recipient == recipient)
        {
            let u = self.commands.memories[actor].delivery.take().unwrap();
            if self.agents[actor].goal == Some((u.rendezvous, false)) {
                self.agents[actor].goal = None;
            }
            self.commands.journal.add("deliveries_completed");
            self.log(
                Some(actor),
                "CommandDeliveryCompleted",
                format!("message={message}; recipient={recipient}"),
            );
        }
    }
    pub(crate) fn cancel_command_delivery(&mut self, actor: usize, reason: &str) {
        if let Some(m) = self.commands.memories.get_mut(actor)
            && let Some(u) = m.delivery.take()
        {
            if self.agents[actor].goal == Some((u.rendezvous, false)) {
                self.agents[actor].goal = None;
            }
            self.commands.journal.add("deliveries_cancelled");
            self.log(
                Some(actor),
                "CommandDeliveryCancelled",
                format!("message={}; reason={reason}", u.message),
            );
        }
    }
    pub(crate) fn audit_commands(&self) -> Result<(), String> {
        if !self.config.commands {
            return if self.commands.memories.is_empty() && self.commands.next_message == 0 {
                Ok(())
            } else {
                Err("disabled commands retained live state".into())
            };
        }
        if !self.config.information
            || self.config.authority.is_none()
            || self.commands.memories.len() != self.agents.len()
        {
            return Err("command prerequisites/cardinality".into());
        }
        let mut originals: BTreeMap<MessageId, &Envelope> = BTreeMap::new();
        for (person, m) in self.commands.memories.iter().enumerate() {
            for (&id, r) in &m.envelopes {
                let p = &r.envelope;
                if id != p.id()
                    || id >= self.commands.next_message
                    || p.issuer() >= self.agents.len()
                    || p.recipient() >= self.agents.len()
                    || r.sender >= self.agents.len()
                    || p.issued_at() > r.received_at
                    || r.received_at > self.now
                    || r.node >= self.config.nodes
                {
                    return Err("invalid command provenance".into());
                }
                if let Some(old) = originals.insert(id, p)
                    && old != p
                {
                    return Err("command mutated in transmission".into());
                }
                if self.commands.memories[p.issuer()]
                    .envelopes
                    .get(&id)
                    .is_none_or(|source| {
                        source.envelope != *p
                            || source.sender != p.issuer()
                            || source.received_at != p.issued_at()
                    })
                {
                    return Err("command lacks issuer original".into());
                }
            }
            for (&id, certificate) in &m.accepted_offices {
                if id != certificate.id
                    || certificate.status != authority::OfficeStatus::Appointed
                    || (certificate.holder != person && certificate.appointed_by != person)
                    || self.authority.offices.get(id) != Some(certificate)
                {
                    return Err("invalid accepted office certificate".into());
                }
            }
            for (&id, offered) in &m.offered_offices {
                if id != offered.id
                    || offered.holder != person
                    || offered.status != authority::OfficeStatus::Offered
                    || self.authority.offices.get(id) != Some(offered)
                {
                    return Err("invalid personally received office offer".into());
                }
            }
            for (&id, site) in &m.known_sites {
                if id != site.project
                    || site.settlement >= self.config.nodes
                    || site.observed_at > self.now
                {
                    return Err("invalid known project site".into());
                }
            }
            if m.pursuing
                .is_some_and(|id| m.obligations.get(&id).is_none_or(|o| o.status.terminal()))
            {
                return Err("active role route without nonterminal obligation".into());
            }
            for (&id, o) in &m.obligations {
                let Some(Receipt {
                    envelope: Envelope::Order(order),
                    received_at,
                    ..
                }) = m.envelopes.get(&id)
                else {
                    return Err("obligation lacks personally received order".into());
                };
                if o.command != id
                    || order.recipient != person
                    || o.objective != order.objective
                    || o.source_issuer != order.issuer
                    || o.recipient_office != order.office
                    || o.received_at != *received_at
                    || o.issued_at != order.issued_at
                    || o.expires_at != order.expires_at
                    || o.changed_at > self.now
                    || !m.accepted_offices.contains_key(&order.office)
                    || m.invalid.contains_key(&id)
                {
                    return Err("invalid obligation provenance".into());
                }
            }
            if let Some(u) = &m.delivery
                && (!m.envelopes.contains_key(&u.message)
                    || u.recipient >= self.agents.len()
                    || u.rendezvous >= self.config.nodes
                    || u.created_at > self.now)
            {
                return Err("invalid command delivery undertaking".into());
            }
        }
        Ok(())
    }
}
