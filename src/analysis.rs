use crate::{pokemon::{self, Pokemon}, team::Team, typing::{combine_defense_charts_immune, BasicType, TypeTrait}};
use itertools::Itertools;
use rand::{seq::SliceRandom, Rng};
use rayon::prelude::*;
use strum::IntoEnumIterator;
use std::{collections::{BTreeMap, HashMap}};

fn score_resistance(team: &Team) -> i32 {
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

fn create_complement_matrix(pool: &Vec<Pokemon>) -> HashMap<Pokemon, HashMap<Pokemon, i32>> {
    let mut result: HashMap<Pokemon, HashMap<Pokemon, i32>> = HashMap::new();
    let combinations = pool.iter().combinations(2);
    for combination in combinations {
        let (p1, p2) = (combination[0], combination[1]);
        result.entry(p1.clone()).or_insert(HashMap::new()).insert(p2.clone(), p1.resistance_complements(p2));
        result.entry(p2.clone()).or_insert(HashMap::new()).insert(p1.clone(), p2.resistance_complements(p1));
    }
    result
}

fn create_compl_team(pokemon: &Pokemon, pool: &Vec<Pokemon>, iterations: usize, team_size: usize) -> Team {
    let mut team: Vec<Pokemon> = vec![pokemon.clone()].into_iter().chain(Team::random(pool.into_iter().cloned(), team_size - 1).pokemon).collect();
    let mut best_team = team.clone();
    let mut best_score = score_resistance(& Team { pokemon: team.clone() });
    for _ in 0..iterations {
        let matrix = create_complement_matrix(&team);
        // 80% chance remove worst complement teammate
        // 20% change remove random teammate
        let mut rng = rand::thread_rng();
        let remove_worst = rng.gen_bool(0.8);
        if remove_worst {
            let worst = team.iter().filter(|p| *p != pokemon)
                .min_by_key(|p| matrix[p].values().sum::<i32>()).unwrap().clone();
            let i = team.iter().position(|p| *p == worst).unwrap();
            team.remove(i);
        } else {
            while team.len() == team_size {
                let random = rng.gen_range(0..team.len());
                team.remove(random);
                if !team.contains(pokemon) {
                    team.push(pokemon.clone());
                }
            }
        }
        team.push(Pokemon::random(&pool.iter().cloned().collect()));
        let score = score_resistance(& Team { pokemon: team.clone() });
        if score > best_score {
            best_score = score;
            best_team = team.clone();
            println!("{score:?} {team:?}");
        }
    }
    println!("{best_score:?} {best_team:?}");
    Team { pokemon: best_team }
}

#[cfg(test)]
mod test {
    use std::{i32, usize};
    use itertools::all;
    use pokemon::Typing;
    use rayon::vec;
    use super::*;

    #[test]
    fn create_complement_matrix_test() {
        let ludicolo = Pokemon { typing: Typing::Dual(BasicType::Grass, BasicType::Water), ability: None };
        let primal_groundon = Pokemon { typing: Typing::Dual(BasicType::Ground, BasicType::Fire), ability: None };
        assert_eq!(ludicolo.resistance_complements(&primal_groundon), 2);
        assert_eq!(primal_groundon.resistance_complements(&ludicolo), 1);
        let matrix = create_complement_matrix(&vec![ludicolo, primal_groundon]);
        println!("{matrix:?}");
    }

    #[test]
    fn get_best_team() {
        let mut max_score = i32::MIN;
        loop {
            let team = Team::random(Pokemon::all_no_abilities(), 6);
            let score = score_resistance(&team);
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
        let score = score_resistance(&team);
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
            let score = score_resistance(&team);
            if score >= max_score {
                println!("{score:?} {team:?}");
                max_score = score;
            }
        }
    }

    #[test]
    fn find_poke_complement() {
        // Given a pokemon, find all pokemon that resist all of its weaknesses
        let ludicolo = Pokemon { typing: Typing::Dual(BasicType::Grass, BasicType::Water), ability: None };
        let complements = ludicolo.find_resistance_complements(Pokemon::all_no_abilities());
        println!("{complements:?}");
    }

    #[test]
    fn create_compl_team_test() {
        let pool = Pokemon::all_no_abilities().collect::<Vec<_>>();
        let team = create_compl_team(&Pokemon { typing: Typing::Dual(BasicType::Grass, BasicType::Water), ability: None }, &pool, 10000, 6);
        print!("{team:?}");
    }
}
