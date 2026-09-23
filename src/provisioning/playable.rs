//! Typed client boundary. Rendering owns snapshots, never mutable authoritative state.
pub mod military;
pub mod session;
pub mod snapshot;

pub use military::MilitaryOrder;
pub use session::{PlayerCommand, ScenarioKind, Session};
pub use snapshot::{Snapshot, ViewMode};

impl Session {
    /// Pure projection; no event, memory observation or command is generated.
    pub fn snapshot(&self, mode: ViewMode) -> Snapshot {
        let mut result = snapshot::capture(self.world(), self.ruler(), mode);
        if mode == ViewMode::Play {
            for node in &mut result.nodes {
                if node.evidence.source == snapshot::KnowledgeSource::Memory
                    && self.map_only_seeds.get(&node.id) == Some(&node.evidence.observed_at)
                {
                    node.contents_known = false;
                    node.board_quantity = None;
                    node.inspect_lines = vec![
                        "Seeded map knowledge only; people, goods and requests unobserved".into(),
                    ];
                }
            }
            for edge in &mut result.edges {
                if edge.evidence.source == snapshot::KnowledgeSource::Memory
                    && edge.queue_scope_node.is_some_and(|node| {
                        self.map_only_seeds.get(&node) == Some(&edge.evidence.observed_at)
                    })
                {
                    edge.queued = None;
                    edge.queue_scope_node = None;
                }
            }
            for order in &mut result.orders {
                if order.issuer == self.ruler()
                    && let Some(handoff) = self.local_handoffs.get(&order.id)
                {
                    match handoff.state {
                        session::DispatchState::DeliveredLocally => {
                            order.received_at = Some(handoff.at);
                            order.status =
                                "Delivered locally; recipient decision unreported".into();
                        }
                        session::DispatchState::WithAgreedCourier => {
                            order.status = format!(
                                "Handed to agreed courier at {}; recipient status unknown",
                                handoff.at
                            );
                        }
                        session::DispatchState::HeldByIssuer => {}
                    }
                }
            }
        }
        result
    }
    pub fn now(&self) -> super::Time {
        self.world().now()
    }
}
