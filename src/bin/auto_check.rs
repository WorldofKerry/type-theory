use std::collections::BTreeSet;
use type_theory::{pokemon::Pokemon, typing::TypeTrait};

/// Builds a team, selecting the pokemon that checks the most opposing pokemon in a pool
fn main() {
    let mut remaining = Pokemon::all_unique_type_chart().collect::<BTreeSet<_>>();
    let mut team = BTreeSet::new();
    let pool: BTreeSet<Pokemon> = Pokemon::from_pkhex_dump("data/Box Data Dump.csv")
        .into_iter()
        .collect();
    while !remaining.is_empty() && team.len() < 6 {
        let best = pool
            .difference(&team)
            .cloned()
            .map(|p1| {
                (
                    remaining
                        .iter()
                        .filter(|p2| {
                            false
                            // Either resist all stabs and hits at least neutrally
                            || (p2.typing.iter().all(|t| p1.defense().get(*t) < 1.0)
                                && p1.typing.iter().any(|t| p2.defense().get(*t) >= 1.0))
                            // Or not weak to any stab and hits back supereffectively
                            || (p2.typing.iter().all(|t| p1.defense().get(*t) <= 1.0)
                                && p1.typing.iter().any(|t| p2.defense().get(*t) > 1.0))
                            // Or at least two members in team and is neutral in both directions
                            || (team.len() >= 2
                                && p2.typing.iter().all(|t| p1.defense().get(*t) <= 1.0)
                                && p1.typing.iter().any(|t| p2.defense().get(*t) >= 1.0))
                        })
                        .cloned()
                        .collect::<BTreeSet<_>>(),
                    p1,
                )
            })
            .max_by_key(|(set, _)| set.len())
            .unwrap();
        remaining = remaining.difference(&best.0).cloned().collect();
        println!(
            "{:?}: checks {:?} {:?}",
            best.1.species,
            best.0.len(),
            best.0
                .iter()
                .map(|p| format!("{:?} {:?}", p.typing, p.ability))
                .collect::<Vec<_>>()
        );
        println!();
        team.insert(best.1);
    }
    println!("remaining: {:?}", remaining);
}
