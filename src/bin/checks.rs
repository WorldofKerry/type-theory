use itertools::Itertools;
use type_theory::{pokemon::Pokemon, typing::TypeTrait};

/// Finds typings that check most other types
/// Check is defined as:
/// - A pokemon that resists all stab moves of the opposing pokemon
/// - A pokemon that has a supereffective stab move against the opposing pokemon
fn main() {
    Pokemon::all_type_combinations()
        .map(|p1| {
            (
                Pokemon::all_type_combinations()
                    .filter(|p2| p2.typing.iter().all(|t| p1.defense().get(*t) < 1.0))
                    .filter(|p2| p1.typing.iter().any(|t| p2.defense().get(*t) > 1.0))
                    .count(),
                p1,
            )
        })
        .sorted()
        .for_each(|(score, p)| {
            println!(
                "{:?} {:?}",
                p.typing,
                score as f64 / Pokemon::all_type_combinations().count() as f64
            );
        });
}
