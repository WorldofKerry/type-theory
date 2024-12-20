use std::collections::BTreeSet;

use strum::IntoEnumIterator;

use crate::{pokemon::Pokemon, typing::{BasicType, TypeTrait}};

// Score how many types the team is able to hit offensively
pub fn score_offensive_coverage(team: &Vec<Pokemon>) -> f64 {
    let team_stabs = team.iter().flat_map(|poke| poke.typing.iter()).collect::<BTreeSet<_>>();
    let mut score = 0.0;
    for t in BasicType::iter() {
        let type_def = Pokemon::from(t).defense();
        if team_stabs.iter().any(|stab| type_def.get(**stab) > 1.0) {
            score += 1.0;
        }
    }
    score
}