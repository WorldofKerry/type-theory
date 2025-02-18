use strum::IntoEnumIterator;

use crate::{pokemon::Pokemon, typing::{BasicType, TypeTrait}};

use super::scoring::reversed_elu;

/// For every type, compute net number of Pokemon resisting and weak to that type
/// Biased towards Pokemon with high resistance-to-weakness ratio, e.g. Water/Ground, Bug/Steel
pub fn one_resist_for_each_type(team: &Vec<Pokemon>) -> f64 {
    let team_defenses = team.iter().map(|poke| poke.defense()).collect::<Vec<_>>();
    let mut score = 0.0;
    for t in BasicType::iter() {
        let count = team_defenses.iter().filter(|def| def.get(t) > 1.0).count();
        for i in 0..count {
            score += 1.0 / (i + 1) as f64 / (i + 1) as f64;
        }
    }
    score
}

/// For every type, score on product of weak/resist multipliers
/// Biased towards Pokemon with immunities
pub fn per_type_multiplier(team: &Vec<Pokemon>, immune_multiplier: f64) -> f64 {
    let team_defenses = team.iter().map(|poke| poke.defense()).collect::<Vec<_>>();
    let mut score = 0.0;
    for t in BasicType::iter() {
        let mut multiplier = 1.0;
        for def in team_defenses.iter() {
            if def.get(t) == 0.0 {
                multiplier *= immune_multiplier;
            } else {
                multiplier *= def.get(t) as f64;
            }
        }
        let normalized = -multiplier.log2();
        let net = reversed_elu(normalized);
        score += net;
    }
    score
}

/// For every type, score on difference between the number of resistances and weaknesses
pub fn per_type_net_resist_weak_count(team: &Vec<Pokemon>) -> f64 {
    let team_defenses = team.iter().map(|poke| poke.defense()).collect::<Vec<_>>();
    let mut score = 0.0;
    for t in BasicType::iter() {
        let weak_count = team_defenses.iter().filter(|def| def.get(t) > 1.0).count();
        let resist_count = team_defenses.iter().filter(|def| def.get(t) < 1.0).count();
        let diff = resist_count as f64 - weak_count as f64;
        let net = reversed_elu(diff);
        score += net;
    }
    score
}
