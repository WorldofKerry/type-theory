use std::collections::{BTreeMap, BTreeSet};
use itertools::Itertools;
use strum::IntoEnumIterator;
use crate::{pokemon::Pokemon, typing::BasicType};

/// Find the number of each type on the team, then score
fn type_distribution(team: &Vec<Pokemon>) -> f64 {
    let type_count = team.iter().flat_map(|poke| poke.typing.iter()).counts();
    let mut score = 0;
    for t in BasicType::iter() {
        let count = type_count.get(&t).unwrap_or(&0);
        score += count * count;
    }
    todo!()
}
