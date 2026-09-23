use super::{HOUR, Time};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Material {
    Food,
    Waste,
    Wood,
    Stone,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Owner {
    None,
    Agent(usize),
    Commons(usize),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Holder {
    MarketEscrow(usize),
    Cart(usize),
    Deposit(usize),
    Loose(usize),
    Agent(usize),
    Commons(usize),
    Edge {
        edge: usize,
        from: usize,
        progress: u128,
    },
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Batch {
    pub id: usize,
    pub origin: usize,
    pub material: Material,
    pub quantity: u64,
    pub owner: Owner,
    pub holder: Holder,
    pub created_at: Time,
    pub spoil_after: Time,
}
impl Batch {
    pub fn expires_at(&self) -> Time {
        self.created_at.saturating_add(self.spoil_after)
    }
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Goods {
    pub batches: Vec<Batch>,
    pub initial: [u128; 3],
    pub consumed: u128,
}
impl Goods {
    pub fn add(
        &mut self,
        material: Material,
        quantity: u64,
        owner: Owner,
        holder: Holder,
        created_at: Time,
        spoil_after: Time,
    ) -> usize {
        let id = self.batches.len();
        self.batches.push(Batch {
            id,
            origin: id,
            material,
            quantity,
            owner,
            holder,
            created_at,
            spoil_after,
        });
        self.initial[match material {
            Material::Food | Material::Waste => 0,
            Material::Wood => 1,
            Material::Stone => 2,
        }] += quantity as u128;
        id
    }
    pub fn move_part(&mut self, id: usize, q: u64, owner: Owner, holder: Holder) -> usize {
        assert!(q > 0 && self.batches[id].quantity >= q);
        let source = self.batches[id].clone();
        self.batches[id].quantity -= q;
        if let Some(b) = self.batches.iter_mut().find(|b| {
            b.id != id
                && b.origin == source.origin
                && b.material == source.material
                && b.owner == owner
                && b.holder == holder
                && b.created_at == source.created_at
                && b.spoil_after == source.spoil_after
        }) {
            b.quantity += q;
            return b.id;
        }
        let target = self.batches.len();
        self.batches.push(Batch {
            id: target,
            quantity: q,
            owner,
            holder,
            ..source
        });
        target
    }
    pub fn load(&self, agent: usize) -> u64 {
        self.batches
            .iter()
            .filter(|b| b.holder == Holder::Agent(agent))
            .map(|b| b.quantity)
            .sum()
    }
    pub fn audit(&self) -> bool {
        self.audit_production(0)
    }
    pub fn audit_production(&self, produced: u128) -> bool {
        self.audit_capital(produced, 0)
    }
    pub fn audit_capital(&self, produced: u128, embedded_wood: u128) -> bool {
        let mut current = [self.consumed, embedded_wood, 0];
        for b in &self.batches {
            current[match b.material {
                Material::Food | Material::Waste => 0,
                Material::Wood => 1,
                Material::Stone => 2,
            }] += b.quantity as u128;
        }
        current == [self.initial[0] + produced, self.initial[1], self.initial[2]]
    }
}
pub(crate) fn headroom(nutrition: Time, owner: Owner) -> u64 {
    let ceiling = if matches!(owner, Owner::Commons(_)) {
        12 * HOUR
    } else {
        24 * HOUR
    };
    ((ceiling - nutrition).max(0) / HOUR) as u64
}
