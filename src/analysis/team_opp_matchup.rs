//! Evaluates a team based on how well it matches up against a random pokemon

use crate::{pokemon::Pokemon, typing::TypeTrait};

pub fn good_matchup(poke: &Pokemon, foe: &Pokemon) -> f64 {
    // Does it resist the opponent's STAB?
    let poke_def = poke.defense();
    let mut count = foe.typing.iter().filter(|t| poke_def.get(**t) > 1.0).count();

    // Does it hit the opponent super effectively?
    let foe_def = foe.defense();
    count += poke.typing.iter().filter(|t| foe_def.get(**t) > 1.0).count();

    count as f64
}

/// Evaluates a team based on how well it matches up against a random pokemon
pub fn score_team_opp_matchup(team: &Vec<Pokemon>, opponents: &Vec<Pokemon>, score_func: fn(&Pokemon, &Pokemon) -> f64) -> f64 {
    let mut score = 0.0;
    for poke in team {
        for foe in opponents {
            score += score_func(poke, foe);
        }
    }
    score
}
