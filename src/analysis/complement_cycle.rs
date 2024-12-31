use crate::pokemon::{Pokemon};
use itertools::Itertools;
use rayon::prelude::*;
use strum::IntoEnumIterator;

use super::complement_matrix::resistance_complements;

fn compute_cyclic_resistance_complement(team: &Vec<Pokemon>) -> i32 {
    let mut score = 0;
    for (poke1, poke2) in team.iter().tuple_windows() {
        score += resistance_complements(poke1, poke2);
    }
    score += resistance_complements(team.last().unwrap(), team.first().unwrap());
    score
}

fn best_complement_cycle(team: &Vec<Pokemon>, pool: &Vec<Pokemon>, size: usize) -> Vec<Pokemon> {
    if team.len() > 5 {
        return team.clone();
    }
    let last_pokemon = team.last().unwrap().clone();
    // Find candidates that complement the last pokemon
    let candidates = pool
        .iter()
        .unique()
        .map(|p| (p.clone(), resistance_complements(&last_pokemon, p)))
        .max_set_by_key(|(_, s)| *s)
        .into_iter()
        .map(|(p, _)| p)
        .collect::<Vec<_>>();
    // Score how well first pokemon complements each candidate, and teams built from there
    let mut best_team = vec![];
    let mut best_score = i32::MIN;
    for candidate in candidates {
        let new_team = team.iter().cloned().chain(vec![candidate]).collect();
        let score1 = compute_cyclic_resistance_complement(&new_team);
        let score2 =
            compute_cyclic_resistance_complement(&best_complement_cycle(&new_team, pool, size));
        if score2 > best_score {
            best_score = score2;
            best_team = best_complement_cycle(&new_team, pool, size);
        }
        if score1 > best_score {
            best_score = score1;
            best_team = new_team;
        }
        println!("{best_score:?} {best_team:?} {best_score:?} {score1:?} {score2:?}");
    }
    best_team
}

#[cfg(test)]
mod tests {
    use crate::typing::BasicType;
    use super::*;
    #[test]
    #[ignore]
    fn recursive_search_specific_type_complements() {
        use BasicType::*;
        let team = best_complement_cycle(&vec![Pokemon::from((Steel, Flying))], Pokemon::all(), 2);
        println!("{team:?}");
    }

    #[test]
    fn resistance_complements_test() {
        use BasicType::*;
        let poke = Pokemon::from((Flying, Steel));
        // Pokemon::all()
        vec![Pokemon::from((Flying, Steel))]
            .into_iter()
            .unique()
            .map(|p| (p.clone(), resistance_complements(&poke, &p)))
            .sorted_by(|(_, s1), (_, s2)| s1.cmp(s2))
            .for_each(|(p, s)| {
                println!("{s:?} {p:?}");
            });
    }
}