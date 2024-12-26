use std::collections::BTreeSet;

use crate::{pokemon::Pokemon, typing::TypeTrait};

fn counters(checker: &Pokemon, checkee: &Pokemon) -> bool {
    // Resists all stabs and hits supereffectively
    (checkee.typing.iter().all(|t| checker.defense().get(*t) < 1.0)
        && checker.typing.iter().any(|t| checkee.defense().get(*t) > 1.0))
}

fn checks(checker: &Pokemon, checkee: &Pokemon) -> bool {
    // Either resist all stabs and hits at least neutrally
    (checkee.typing.iter().all(|t| checker.defense().get(*t) < 1.0)
    && checker.typing.iter().any(|t| checkee.defense().get(*t) >= 1.0))
    // Or not weak to any stab and hits back supereffectively
    || (checkee.typing.iter().all(|t| checker.defense().get(*t) <= 1.0)
        && checker.typing.iter().any(|t| checkee.defense().get(*t) > 1.0))
}

pub fn counter_count(team: &Vec<Pokemon>, pool: &BTreeSet<Pokemon>) -> usize {
    pool.iter()
        .filter(|p1| {
            team.iter().any(|p2| counters(p2, p1))
        })
        .count()
}

pub fn checks_count(team: &Vec<Pokemon>, pool: &BTreeSet<Pokemon>) -> usize {
    pool.iter()
        .filter(|p1| {
            team.iter().any(|p2| checks(p2, p1))
        })
        .count()
}