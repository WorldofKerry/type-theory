use std::collections::BTreeSet;

use crate::{pokemon::Pokemon, typing::TypeTrait};

pub fn counters(checker: &Pokemon, checkee: &Pokemon) -> bool {
    // Resists all stabs and hits supereffectively
    checkee.typing.iter().all(|t| checker.defense().get(*t) < 1.0)
        && checker.typing.iter().any(|t| checkee.defense().get(*t) > 1.0)
}

pub fn checks(checker: &Pokemon, checkee: &Pokemon) -> bool {
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

/// Return uncountered counters
pub fn counter_balance(team: &Vec<Pokemon>) -> Vec<Pokemon> {
    let opposing_checks = Pokemon::all_unique_type_chart().iter().filter(
        |p| team.iter().any(|t| counters(p, t))
    );
    opposing_checks.into_iter().filter(
        |p| !team.iter().any(|t| counters(t, p))
    ).cloned().collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::typing::BasicType::*;
    #[test]
    fn test_main() {
        assert!(!checks(&Pokemon::from((Ground, Rock)), &Pokemon::from((Fire, Steel))));
        assert!(counters(&Pokemon::from((Steel, Ground)), &Pokemon::from(Poison)));
        assert!(!counters(&Pokemon::from(Water), &Pokemon::from(Ground)));
        assert!(counters(&Pokemon::from(Grass), &Pokemon::from(Ground)));
        assert!(!counters(&Pokemon::from(Flying), &Pokemon::from(Ground)));
    }

    #[test]
    fn test_balance() {
        let team = vec![
            Pokemon::from((Flying, Ground)),
            Pokemon::from((Rock, Ghost)),
        ];
        let balance = counter_balance(&team);
        println!("{:?}", balance);
    }
}