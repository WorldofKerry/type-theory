use strum::IntoEnumIterator;

use crate::{pokemon::Pokemon, typing::{BasicType, TypeTrait}};

/// Favours teams that have at least one resistance to each type
pub fn resistance_count(team: &Vec<Pokemon>) -> f64 {
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

/// Favours teams that have a net resistance multipler to each type greater than 1
pub fn resistance_multiplier(team: &Vec<Pokemon>, immune_multiplier: f64) -> f64 {
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
        score += -multiplier;
    }
    score
}

/// Favour teams that have more members weak to a type than resistant
pub fn resistance_balance(team: &Vec<Pokemon>) -> f64 {
    let team_defenses = team.iter().map(|poke| poke.defense()).collect::<Vec<_>>();
    let mut score = 0.0;
    for t in BasicType::iter() {
        let weak_count = team_defenses.iter().filter(|def| def.get(t) > 1.0).count();
        let resist_count = team_defenses.iter().filter(|def| def.get(t) < 1.0).count();
        let diff = resist_count as f64 - weak_count as f64;
        if diff <= 0.0 {
            // Linear penalty for more weaknesses
            score += diff;
        } else {
            // Logarithmic reward for more resistances
            score += 1.0 - 1.0 / (diff + 1.0);
        }
    }
    score
}