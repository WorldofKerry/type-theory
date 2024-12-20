//! Evaluates a team based on how well it matches up against a random pokemon

use crate::{pokemon::Pokemon, typing::TypeTrait};

fn good_matchup(poke: &Pokemon, foe: &Pokemon) -> f64 {
    // Does it resist the opponent's STAB?
    let poke_def = poke.defense();
    let mut count = foe.typing.iter().filter(|t| poke_def.get(**t) > 1.0).count();

    // Does it hit the opponent super effectively?
    let foe_def = foe.defense();
    count += poke.typing.iter().filter(|t| foe_def.get(**t) > 1.0).count();

    count as f64
}

/// Evaluates a team based on how well it matches up against a random pokemon
fn team_opp_matchup(team: Vec<Pokemon>, opponents: Vec<Pokemon>, score_func: fn(&Pokemon, &Pokemon) -> f64) -> f64 {
    let mut score = 0.0;
    for poke in &team {
        for foe in &opponents {
            score += score_func(poke, foe);
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use crate::team::Team;

    use super::*;

    // #[test]
    // fn find_good_team() {
    //     let mut best_score = 0.0;

    //     loop {
    //         let team = Team::random(Pokemon::all().into_iter(), 6);

    //         let opponents = (0..100).map(|_| Pokemon::random(&Pokemon::all_type_combinations_and_abilities().collect::<Vec<_>>())).collect();

    //         let score = team_opp_matchup(team.pokemon.clone(), opponents, good_matchup);

    //         if score > best_score {
    //             best_score = score;
    //             let best_team = team.pokemon.clone();
    //             println!("New best score: {}", best_score);
    //             for poke in &best_team {
    //                 println!("{:?}", poke);
    //             }
    //         }
    //     }
    // }
}