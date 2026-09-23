//! Observation/storage adapter. World truth stops here, before pure choice.
use super::*;
use crate::provisioning::{World, authority};
impl World {
    pub(crate) fn person_capabilities(&self, id: usize) -> Capabilities<'_> {
        Capabilities {
            cultivation: self
                .config
                .farming
                .as_ref()
                .map(|c| (&self.farm.memories[id], &c.rules)),
            capital: self
                .config
                .capital
                .as_ref()
                .map(|c| (&self.capital.memories[id], &c.rules)),
            public_works: self
                .config
                .infrastructure
                .as_ref()
                .map(|c| (&self.infrastructure.memories[id], c.enabled)),
        }
    }
    /// Personal state inspection does not refresh any beliefs. No remote resources,
    /// current foreign offices, or global ownership inventory enter this view.
    pub fn person_state(&self, id: usize) -> Option<State<'_>> {
        let a = self.agents.get(id)?;
        let mut roles = vec![];
        if let Some(p) = self.polity()
            && p.sovereign == id
        {
            roles.push(Role::Sovereign { polity: p.id });
        }
        for o in self
            .offices()
            .iter()
            .filter(|o| o.holder == id && o.status == authority::OfficeStatus::Appointed)
        {
            roles.push(Role::Steward {
                polity: o.polity,
                office: o.id,
                settlement: o.settlement,
            });
        }
        Some(State {
            person: a,
            needs: a.needs(),
            reasoning: a.reasoning_state(),
            capabilities: self.person_capabilities(id),
            roles,
            rights: Rights {
                commons_membership: self.config.communal.then_some(0),
                voluntary_goods_exchange: self.config.barter,
                personal_crop_claims: self.config.farming.is_some(),
                personal_cart_use: self.config.capital.is_some(),
            },
            observation: self.observe(id),
        })
    }
}
