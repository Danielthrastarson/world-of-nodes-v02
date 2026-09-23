//! Local kernel endpoints. Origin observations are copied, never refreshed remotely.
use super::*;

impl World {
    pub fn information_memory(&self, id: usize) -> Option<&Memory> {
        self.information.memories.get(id)
    }
    pub fn information_journal(&self) -> &Journal {
        &self.information.journal
    }
    pub(crate) fn information_actor_node(&self, id: usize) -> Result<usize, String> {
        if !self.config.information {
            return Err("information institution disabled".into());
        }
        let a = self.agents.get(id).ok_or("unknown person")?;
        if a.action == Action::Dead || self.body_factor(id) == 0 {
            return Err("living capable person required".into());
        }
        let Location::Node(node) = a.location else {
            return Err("communication requires physical node presence".into());
        };
        Ok(node)
    }
    /// Controlled local observation endpoint. The caller cannot provide a remote node
    /// or fabricate report contents; the report is the person's actual current view.
    pub fn capture_report(&mut self, id: usize) -> Result<ReportId, String> {
        self.information_actor_node(id)?;
        let observation = self.observe(id).unwrap().local.unwrap();
        let report = Report {
            id: self.information.next_report,
            author: id,
            observation,
        };
        self.information.next_report += 1;
        self.agents[id]
            .memory
            .nodes
            .insert(report.observation.node, report.observation.clone());
        self.information.memories[id]
            .reported_nodes
            .remove(&report.observation.node);
        self.information.memories[id].reports.insert(
            report.id,
            Receipt {
                report: report.clone(),
                sender: id,
                received_at: self.now,
            },
        );
        self.information.journal.add("reports_created");
        self.log(
            Some(id),
            "ReportCreated",
            format!(
                "report={}; node={}; observed_at={}; actual local observation",
                report.id, report.observation.node, report.observation.observed_at
            ),
        );
        Ok(report.id)
    }
    /// Oral copy at physical contact. Information can be copied; material goods are
    /// neither created nor transported by this operation. Knowledge confers no rights.
    pub fn receive_report(
        &mut self,
        sender: usize,
        recipient: usize,
        report: ReportId,
    ) -> Result<(), String> {
        let node = self.information_actor_node(sender)?;
        if sender == recipient {
            return Err("self handoff is not communication".into());
        }
        if self.information_actor_node(recipient)? != node {
            return Err("recipient not physically colocated".into());
        }
        let payload = self.information.memories[sender]
            .reports
            .get(&report)
            .ok_or("sender does not hold report")?
            .report
            .clone();
        if self.information.memories[recipient]
            .reports
            .contains_key(&report)
        {
            self.information.journal.add("duplicate_handoffs");
            self.cancel_action(sender, "local report handoff already known");
            self.log(
                Some(sender),
                "ReportDuplicate",
                format!("report={report}; recipient={recipient}; original receipt retained"),
            );
            self.complete_report_delivery(sender, report, recipient);
            return Ok(());
        }
        let observed = &payload.observation;
        let updated = self.agents[recipient]
            .memory
            .nodes
            .get(&observed.node)
            .is_none_or(|old| old.observed_at < observed.observed_at);
        if updated {
            self.agents[recipient]
                .memory
                .nodes
                .insert(observed.node, observed.clone());
            self.information.memories[recipient]
                .reported_nodes
                .insert(observed.node, report);
        }
        let h = Handoff {
            report,
            author: payload.author,
            sender,
            recipient,
            node,
            observed_at: observed.observed_at,
            received_at: self.now,
            spatial_belief_updated: updated,
        };
        self.information.memories[recipient].reports.insert(
            report,
            Receipt {
                report: payload,
                sender,
                received_at: self.now,
            },
        );
        self.information.journal.add("handoffs");
        self.information.journal.add(if updated {
            "belief_updates"
        } else {
            "old_or_equal_reports_retained"
        });
        self.information.journal.handoffs.push(h.clone());
        self.cancel_action(sender, "local report handoff");
        self.log(Some(sender), "ReportHandedOver", format!("{h:?}"));
        self.log(
            Some(recipient),
            "ReportReceived",
            format!("{h:?}; original observation age preserved; no authority transferred"),
        );
        self.complete_report_delivery(sender, report, recipient);
        // Receipt changes evidence, not the recipient's already-selected command.
        // Reconsider at the next normal policy opportunity. A pending wake here
        // would make an arbitrary advance_to checkpoint affect reaction latency.
        Ok(())
    }
    /// Research-controlled initial purpose, not autonomous recruitment or tracking.
    /// A fixed rendezvous is supplied explicitly; its truth is never looked up.
    pub fn begin_report_delivery(
        &mut self,
        actor: usize,
        report: ReportId,
        recipient: usize,
        rendezvous: usize,
    ) -> Result<(), String> {
        self.information_actor_node(actor)?;
        if actor == recipient || recipient >= self.agents.len() || rendezvous >= self.config.nodes {
            return Err("invalid report delivery objective".into());
        }
        let m = &mut self.information.memories[actor];
        if m.delivery.is_some() || !m.reports.contains_key(&report) {
            return Err("already delivering or report not held".into());
        }
        m.delivery = Some(Undertaking {
            report,
            recipient,
            rendezvous,
            created_at: self.now,
        });
        self.information.journal.add("deliveries_started");
        self.log(
            Some(actor),
            "ReportDeliveryStarted",
            format!(
                "report={report}; recipient={recipient}; rendezvous={rendezvous}; controlled purpose, no recipient location queried"
            ),
        );
        // The controlled purpose becomes eligible at the next normal policy turn.
        Ok(())
    }
    fn complete_report_delivery(&mut self, actor: usize, report: ReportId, recipient: usize) {
        if self.information.memories[actor]
            .delivery
            .as_ref()
            .is_some_and(|u| u.report == report && u.recipient == recipient)
        {
            let rendezvous = self.information.memories[actor]
                .delivery
                .as_ref()
                .unwrap()
                .rendezvous;
            self.information.memories[actor].delivery = None;
            if self.agents[actor].goal == Some((rendezvous, false)) {
                self.agents[actor].goal = None;
            }
            self.information.journal.add("deliveries_completed");
            self.log(
                Some(actor),
                "ReportDeliveryCompleted",
                format!("report={report}; recipient={recipient}; physical contact"),
            );
        }
    }
    pub(crate) fn cancel_report_delivery(&mut self, id: usize, reason: &str) {
        if let Some(m) = self.information.memories.get_mut(id)
            && let Some(u) = m.delivery.take()
        {
            if self.agents[id].goal == Some((u.rendezvous, false)) {
                self.agents[id].goal = None;
                if matches!(self.agents[id].action, Action::Queued { .. }) {
                    self.cancel_action(id, "report delivery cancelled");
                }
            }
            self.information.journal.add("deliveries_cancelled");
            self.log(
                Some(id),
                "ReportDeliveryCancelled",
                format!(
                    "report={}; recipient={}; reason={reason}",
                    u.report, u.recipient
                ),
            );
        }
    }
    pub(crate) fn audit_information(&self) -> Result<(), String> {
        if !self.config.information {
            if !self.information.memories.is_empty() || self.information.next_report != 0 {
                return Err("disabled information retained live state".into());
            }
            return Ok(());
        }
        if self.information.memories.len() != self.agents.len() {
            return Err("information memory cardinality".into());
        }
        let mut originals: BTreeMap<ReportId, &Report> = BTreeMap::new();
        for (id, m) in self.information.memories.iter().enumerate() {
            for (&key, r) in &m.reports {
                let p = &r.report;
                if p.id != key
                    || p.id >= self.information.next_report
                    || p.author >= self.agents.len()
                    || r.sender >= self.agents.len()
                    || p.observation.node >= self.config.nodes
                    || p.observation.observed_at > r.received_at
                    || r.received_at > self.now
                    || !p.observation.people.contains(&p.author)
                {
                    return Err("invalid report provenance/time/location".into());
                }
                if let Some(old) = originals.insert(key, p)
                    && old != p
                {
                    return Err("report mutated during transmission".into());
                }
            }
            for (&node, &report) in &m.reported_nodes {
                let Some(r) = m.reports.get(&report) else {
                    return Err("reported spatial belief lacks a personal receipt".into());
                };
                if r.report.observation.node != node
                    || r.report.observation.observed_at > self.now
                    || self.agents[id].memory.nodes.get(&node) != Some(&r.report.observation)
                {
                    return Err("reported spatial belief has invalid provenance".into());
                }
            }
            if let Some(u) = &m.delivery
                && (u.recipient == id
                    || u.recipient >= self.agents.len()
                    || u.rendezvous >= self.config.nodes
                    || u.created_at > self.now
                    || !m.reports.contains_key(&u.report))
            {
                return Err("invalid personal report delivery undertaking".into());
            }
        }
        Ok(())
    }
}
