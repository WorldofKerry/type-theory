use std::collections::BTreeSet;

use strum::IntoEnumIterator;

use crate::{analysis::scoring::reversed_elu, pokemon::Pokemon, typing::{BasicType, TypeTrait}};

// Score how many types the team is able to hit offensively
pub fn offensive_coverage(team: &Vec<Pokemon>) -> f64 {
    let team_stabs = team.iter().flat_map(|poke| poke.typing.iter()).collect::<BTreeSet<_>>();
    let mut score = 0.0;
    for t in BasicType::iter() {
        let type_def = Pokemon::from(t).defense();
        let count = team_stabs.iter().filter(|stab| type_def.get(***stab) > 1.0).count();
        let net = reversed_elu(count as f64);
        // println!("{t:?} {count} {net}");
        score += net;
    }
    score
}