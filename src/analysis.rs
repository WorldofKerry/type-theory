use crate::{pokemon::{self, Pokemon}, typing::combine_defense_charts_immune};
use rand::seq::SliceRandom;
use rayon::prelude::*;
use std::{collections::HashMap, mem::uninitialized};
use crate::{pokemon::Typing, typing::{Ability, BasicType, Relationship, TypeTrait}};
use itertools::Itertools;

#[derive(Clone, Debug)]
struct Team {
    pub pokemon: Vec<Pokemon>,
}

impl Team {
    fn all(size: usize) -> impl Iterator<Item = Team> {
        Pokemon::all_no_abilities().combinations(size).map(move |team| Team { pokemon: team.into_iter().map(|p| p.clone()).collect()})
    }

    fn random(size: usize) -> Team {
        let pokemon = Pokemon::all_no_abilities().collect::<Vec<_>>();
        let mut rng = rand::thread_rng();
        let team = (0..size).map(|_| pokemon.choose(&mut rng).unwrap().clone()).collect();
        Team { pokemon: team }
    }
}

/// Checks the number of pokemon on the team that has another pokemon that resists all of its weaknesses
fn resistance_coverage(team: &Team) -> i32 {
    let combined_charts = combine_defense_charts_immune(team.pokemon.iter().map(|p| p.defense()), 0.25);
    let mut count = 0;
    for (_, v) in combined_charts.iter() {
        if *v > 1.0 {
            count -= 1 * 100000000;
        }
    }
    for (_, v) in combined_charts.iter() {
        if *v < 1.0 {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_best_defensive_pokemon() {
        let teams = Team::all(2).collect::<Vec<_>>();
        let mut results = teams.into_par_iter().map(|team| (team.clone(), resistance_coverage(&team))).collect::<Vec<_>>();

        results.sort_by(|(_, a), (_, b)| b.cmp(a));

        // print top 10
        for (team, score) in results.into_iter().take(10) {
            println!("Score: {}", score);
            for p in team.pokemon {
                println!("{:?}", p);
            }
            println!();
        }
    }

    #[test]
    fn get_best_team() {
        loop {
            let team = Team::random(3);
            let score = resistance_coverage(&team);
            if score >= 14 {
                println!("{score:?} {team:?}");
            }
        }
    }
}
