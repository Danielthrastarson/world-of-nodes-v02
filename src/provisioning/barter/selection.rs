//! Pure exchange stage of the common person policy. No World, no counterparty
//! private needs, no current remote goods, and no research classifier inputs.
use super::{Action, Decision, Intent, Material, Observation, View, willingness};
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub(crate) struct Input {
    pub local: View,
    /// Own locally posted offer, including actually observable escrow backing.
    pub owner_views: BTreeMap<usize, View>,
    pub market: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Note {
    Observed(usize),
    Deferred(usize, &'static str),
    Count(&'static str),
    Declined(usize, &'static str),
}
pub(crate) struct Selection {
    pub decision: Decision,
    pub notes: Vec<Note>,
}
pub(crate) fn choose(input: &Input, o: &Observation, fallback: Decision) -> Selection {
    let v = &input.local;
    let mut notes: Vec<_> = v
        .offers
        .iter()
        .filter(|f| f.proposer != v.agent)
        .map(|f| Note::Observed(f.id))
        .collect();
    if o.action != Action::Idle
        || o.job.is_some()
        || matches!(
            fallback.intent,
            Intent::Eat { .. }
                | Intent::Sleep
                | Intent::Deliver { .. }
                | Intent::GiveCareFood { .. }
                | Intent::TravelForCare { .. }
        )
    {
        if input.market {
            let why = if o.action != Action::Idle {
                "DEFERRED_BUSY"
            } else if o.job.is_some() {
                "DEFERRED_JOB"
            } else {
                "DEFERRED_PRIORITY"
            };
            notes.extend(
                v.offers
                    .iter()
                    .filter(|f| f.proposer != v.agent)
                    .map(|f| Note::Deferred(f.id, why)),
            );
        }
        return Selection {
            decision: fallback,
            notes,
        };
    }
    notes.push(Note::Count("eligible_evaluations"));
    if v.needs.wood > 0 {
        notes.push(Note::Count("missing_wood_plan_evaluations"));
    }
    if v.needs.food > 0 {
        notes.push(Note::Count("unmet_food_evaluations"));
    }
    for f in &v.offers {
        if f.proposer == v.agent {
            let own = input.owner_views.get(&f.id).unwrap_or(v);
            if willingness(own, f.give, f.quantity, f.want, f.want_quantity).is_err() {
                return Selection {
                    decision: Decision {
                        intent: Intent::RevokeBarter { offer: f.id },
                        reason: format!("own use changed; needs={:?}", v.needs),
                    },
                    notes,
                };
            }
            continue;
        }
        let payment = v
            .own
            .iter()
            .find(|b| b.material == f.want && b.quantity >= f.want_quantity);
        let reason = if v.needs.demand(f.give) == 0 {
            Some("COUNTERPARTY_DOES_NOT_WANT_GOOD")
        } else if payment.is_none() {
            Some("INSUFFICIENT_OWNED_QUANTITY")
        } else {
            willingness(v, f.want, f.want_quantity, f.give, f.quantity).err()
        };
        if let Some(reason) = reason {
            notes.push(Note::Declined(f.id, reason));
        } else {
            return Selection {
                decision: Decision {
                    intent: Intent::AcceptBarter {
                        offer: f.id,
                        payment: payment.unwrap().id,
                    },
                    reason: format!(
                        "positive own use; surrender outside protected uses; needs={:?}",
                        v.needs
                    ),
                },
                notes,
            };
        }
    }
    if v.offers.iter().any(|f| f.proposer == v.agent) {
        return Selection {
            decision: fallback,
            notes,
        };
    }
    notes.push(Note::Count("formation_evaluations"));
    for (give, want, limit) in [
        (Material::Food, Material::Wood, 6),
        (Material::Wood, Material::Food, 10),
    ] {
        let q = v.needs.surplus(give).min(limit);
        let ask = v.needs.demand(want);
        if q == 0 || ask == 0 {
            continue;
        }
        if let Some(b) = v.own.iter().find(|b| b.material == give) {
            let q = q.min(b.quantity);
            if v.people.len() < 2 {
                notes.push(Note::Count("offer_without_counterparty"));
            }
            return Selection {
                decision: Decision {
                    intent: Intent::OfferBarter {
                        batch: b.id,
                        quantity: q,
                        want,
                        want_quantity: ask,
                    },
                    reason: format!(
                        "blocked own use; surplus={q}; no resale assumption; needs={:?}",
                        v.needs
                    ),
                },
                notes,
            };
        }
    }
    Selection {
        decision: fallback,
        notes,
    }
}
