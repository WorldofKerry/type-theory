use std::collections::BTreeSet;
use type_theory::{injest::parse_pkhex_dump, pokemon::Pokemon, typing::TypeTrait};

/// Builds a team, selecting the pokemon that checks the most opposing pokemon in a pool
fn main() {
    return; // don't think this is useful anymore, can delete?
    let team_size = 6;
    let counter_weight = 0.5;
    let check_weight = 1.0 - counter_weight;

    let mut remaining_not_countered: BTreeSet<Pokemon> = Pokemon::all_unique_type_chart().iter().cloned().collect();
    let mut remaining_not_checked: BTreeSet<Pokemon> = Pokemon::all_unique_type_chart().iter().cloned().collect();

    let mut team = BTreeSet::new();
    let pool: BTreeSet<Pokemon> = parse_pkhex_dump("Box Data Dump.csv")
        .into_iter()
        .collect();

    while (!remaining_not_countered.is_empty() || !remaining_not_checked.is_empty())
        && team.len() < team_size
    {
        let best = pool
            .difference(&team)
            .cloned()
            .map(|p1| {
                (
                    remaining_not_countered
                        .iter()
                        .filter(|p2| {
                            false
                            // Either resist all stabs and hits at least neutrally
                            || (p2.typing.iter().all(|t| p1.defense().get(*t) < 1.0)
                                && p1.typing.iter().any(|t| p2.defense().get(*t) >= 1.0))
                            // Or not weak to any stab and hits back supereffectively
                            || (p2.typing.iter().all(|t| p1.defense().get(*t) <= 1.0)
                                && p1.typing.iter().any(|t| p2.defense().get(*t) > 1.0))
                        })
                        .cloned()
                        .collect::<BTreeSet<_>>(),
                    remaining_not_checked
                        .iter()
                        .filter(|p2| {
                            false
                            // Neutral in both directions
                            || (p2.typing.iter().all(|t| p1.defense().get(*t) <= 1.0)
                                && p1.typing.iter().any(|t| p2.defense().get(*t) >= 1.0))
                        })
                        .cloned()
                        .collect::<BTreeSet<_>>(),
                    p1,
                )
            })
            .max_by_key(|(counter_set, check_set, _)| {
                (counter_set.len() as f64 * 100.0 * counter_weight
                    + check_set.len() as f64 * 100.0 * check_weight) as i32
            })
            .unwrap();
        remaining_not_countered = remaining_not_countered
            .difference(&best.0)
            .cloned()
            .collect();
        remaining_not_checked = remaining_not_checked
            .difference(&best.1)
            .cloned()
            .collect();
        if best.0.len() + best.1.len() <= 10 {
            println!(
                "{:?}: counters {:?} checks {:?}, counters {:?} checks {:?}",
                best.2.species,
                best.0.len(),
                best.1.len(),
                best.0.iter().map(|p| format!("{:?} {:?}", p.typing, p.ability)).collect::<Vec<_>>(),
                best.1.iter().map(|p| format!("{:?} {:?}", p.typing, p.ability)).collect::<Vec<_>>(),
            );
        } else {
            println!(
                "{:?}: counters {:?} checks {:?}",
                best.2.species,
                best.0.len(),
                best.1.len(),
            );
        }
        println!();
        team.insert(best.2);
    }
    println!("remaining not countered {:?}, not checked {:?} {:?}", remaining_not_countered.len(), remaining_not_checked.len(), remaining_not_checked);
}
