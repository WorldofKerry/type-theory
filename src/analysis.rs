use crate::{pokemon::{self, Pokemon}, typing::combine_defense_charts_immune};
use rand::seq::SliceRandom;
use rayon::prelude::*;
use strum::IntoEnumIterator;
use std::{collections::{BTreeMap, HashMap}, mem::uninitialized};
use crate::{pokemon::Typing, typing::{Ability, BasicType, Relationship, TypeTrait}};
use itertools::Itertools;

#[derive(Clone, Debug)]
struct Team {
    pub pokemon: Vec<Pokemon>,
}

impl Team {
    fn all(pool: impl Iterator<Item = Pokemon>, size: usize) -> impl Iterator<Item = Team> {
        pool.combinations(size).map(move |team| Team { pokemon: team.into_iter().map(|p| p.clone()).collect()})
    }

    fn random(pool: impl Iterator<Item = Pokemon>, size: usize) -> Team {
        let pokemon = pool.collect::<Vec<_>>();
        let mut rng = rand::thread_rng();
        let team = (0..size).map(|_| pokemon.choose(&mut rng).unwrap().clone()).collect();
        Team { pokemon: team }
    }

    fn fill_random(&self, pool: impl Iterator<Item = Pokemon>, size: usize) -> Team {
        let missing = size - self.pokemon.len();
        Team { pokemon: Team::random(pool, missing).pokemon.into_iter().chain(self.pokemon.iter().map(|p| p.clone())).collect() }
    }
}

fn resistance_coverage(team: &Team) -> i32 {
    let mut score = 0;
    let combined = combine_defense_charts_immune(team.pokemon.iter().map(|p| p.defense()), 0.25);
    let mut weakness_count: BTreeMap<BasicType, i32> = BTreeMap::new();
    let mut resistance_count: BTreeMap<BasicType, i32> = BTreeMap::new();
    for t in BasicType::iter() {
        weakness_count.insert(t, 0);
        resistance_count.insert(t, 0);
    }
    for p in &team.pokemon {
        let defense = p.defense();
        for (t, r) in defense.iter() {
            if *r > 1.0 {
                *weakness_count.get_mut(&t).unwrap() += 1;
            } else if *r < 1.0 {
                *resistance_count.get_mut(&t).unwrap() += 1;
            }
        }
    }
    let mut table = BTreeMap::new();
    for t in BasicType::iter() {
        table.insert(t, combined.get(t) * combined.get(t) * weakness_count[&t] as f32 / resistance_count[&t] as f32);
    }
    for (t, r) in combined.iter() {
        if *r > 1.0 {
            score -= 2
        } else if *r < 1.0 {
            score += 1
        }
    }
    score
}

#[cfg(test)]
mod test {
    use std::i32;
    use super::*;

    #[test]
    fn get_best_team() {
        let mut max_score = i32::MIN;
        loop {
            let team = Team::random(Pokemon::all_no_abilities(), 4);
            let score = resistance_coverage(&team);
            if score >= max_score {
                println!("{score:?} {team:?}");
                max_score = score;
            }
        }
    }

    #[test]
    fn specific_team() {
        let team = Team { pokemon: vec![
            Pokemon { typing: Typing::Dual(BasicType::Fire, BasicType::Ground), ability: None },
            Pokemon { typing: Typing::Dual(BasicType::Steel, BasicType::Flying), ability: None },
            Pokemon { typing: Typing::Dual(BasicType::Grass, BasicType::Water), ability: None },
        ]};
        let score = resistance_coverage(&team);
        println!("{score:?} {team:?}");
    }

    #[test]
    fn complementary_members() {
        let fixed_team = Team { pokemon: vec![
            Pokemon { typing: Typing::Dual(BasicType::Dragon, BasicType::Ground), ability: None },
        ]};
        let mut max_score = i32::MIN;
        loop {
            let team = fixed_team.fill_random(Pokemon::all_no_abilities(), 3);
            let score = resistance_coverage(&team);
            if score >= max_score {
                println!("{score:?} {team:?}");
                max_score = score;
            }
        }
    }
}
