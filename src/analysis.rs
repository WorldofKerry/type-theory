use crate::{
    pokemon::{self, Pokemon},
    team::Team,
    typing::{combine_defense_charts_immune, BasicType, TypeTrait},
};
use itertools::Itertools;
use rand::{seq::SliceRandom, Rng};
use rayon::prelude::*;
use std::collections::{BTreeMap, HashMap};
use strum::IntoEnumIterator;

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
        table.insert(
            t,
            combined.get(t) * combined.get(t) * weakness_count[&t] as f32
                / (resistance_count[&t] as f32 + 1.0),
        );
    }
    for (t, r) in combined.iter() {
        let scaled = if *r >= 1.0 {
            r.log2()
        } else {
            -(1.0 / r).log2()
        };
        // println!("{t:?} {r:?} {scaled:?}");

        // if *r > 1.0 {
        //     score -= 2
        // } else if *r < 1.0 {
        //     score += 1
        // }
        score -= scaled as i32;
    }
    score
}

fn create_complement_matrix(pool: &Vec<Pokemon>) -> HashMap<Pokemon, HashMap<Pokemon, i32>> {
    let mut result: HashMap<Pokemon, HashMap<Pokemon, i32>> = HashMap::new();
    let combinations = pool.iter().combinations(2);
    for combination in combinations {
        let (p1, p2) = (combination[0], combination[1]);
        result
            .entry(p1.clone())
            .or_insert(HashMap::new())
            .insert(p2.clone(), resistance_complements(p1, p2));
        result
            .entry(p2.clone())
            .or_insert(HashMap::new())
            .insert(p1.clone(), resistance_complements(p2, p1));
    }
    result
}

fn create_compl_team(
    required: &Pokemon,
    pool: &Vec<Pokemon>,
    iterations: usize,
    team_size: usize,
) -> Team {
    let mut team: Vec<Pokemon> = vec![required.clone()]
        .into_iter()
        .chain(Team::random(pool.into_iter().cloned(), team_size - 1).pokemon)
        .collect();
    let mut best_team = team.clone();
    let mut best_score = score_resistance(&Team {
        pokemon: team.clone(),
    });
    for _ in 0..iterations {
        let matrix = create_complement_matrix(&team);
        // 80% chance remove worst complement teammate
        // 20% change remove random teammate
        let mut rng = rand::thread_rng();
        let remove_worst = rng.gen_bool(0.8);
        if remove_worst {
            let worst = team
                .iter()
                .filter(|p| *p != required)
                .min_by_key(|p| matrix[p].values().sum::<i32>())
                .unwrap()
                .clone();
            let i = team.iter().position(|p| *p == worst).unwrap();
            team.remove(i);
        } else {
            while team.len() == team_size {
                let random = rng.gen_range(0..team.len());
                team.remove(random);
                if !team.contains(required) {
                    team.push(required.clone());
                }
            }
        }
        team.push(Pokemon::random(&pool.iter().cloned().collect()));
        let score = score_resistance(&Team {
            pokemon: team.clone(),
        });
        if score > best_score {
            best_score = score;
            best_team = team.clone();
            println!("{score:?} {team:?}");
        }
    }
    println!("{best_score:?} {best_team:?}");
    Team { pokemon: best_team }
}

pub fn resistance_complements(poke1: &Pokemon, poke2: &Pokemon) -> i32 {
    // How well poke1 resists weaknesses of poke2
    // Higher is better
    const IMMUNITY_LOG: i32 = -2; // treat immunity as 1/8 resistance
    let poke1_def = poke1.defense();
    let poke2_def = poke2.defense();
    let mut score = 0;
    for t in BasicType::iter() {
        let r1 = if poke1_def.get(t) == 0.0 {
            IMMUNITY_LOG
        } else {
            poke1_def.get(t).log2() as i32
        };
        let r2 = if poke2_def.get(t) == 0.0 {
            IMMUNITY_LOG
        } else {
            poke2_def.get(t).log2() as i32
        };
        if r2 > 0 {
            // println!("{t:?} {r1:?} {r2:?}");
            if r1 > 0 {
                match (r2, r1) {
                    (2, 2) => score -= 4,
                    (1, 2) => score -= 3,
                    (2, 1) => score -= 2,
                    (1, 1) => score -= 1,
                    _ => panic!("{r1:?} {r2:?}"),
                }
            } else {
                score -= r1;
            }
        }
    }
    score
}

pub fn resistance_connector(
    poke1: &Pokemon,
    poke2: &Pokemon,
    pool: &Vec<Pokemon>,
) -> Vec<(Pokemon, i32, i32)> {
    pool.iter()
        .map(|poke3| {
            let score1 = resistance_complements(poke3, poke1);
            let score2 = resistance_complements(poke2, poke3);
            (poke3.clone(), score1, score2)
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools::all;
    use pokemon::Typing;
    use rayon::vec;
    use std::{i32, usize};

    #[test]
    fn create_complement_matrix_test() {
        use BasicType::*;
        let ludicolo = Pokemon::from((Grass, Water));
        let primal_groundon = Pokemon::from((Ground, Fire));
        assert_eq!(resistance_complements(&ludicolo, &primal_groundon), 3);
        assert_eq!(resistance_complements(&primal_groundon, &ludicolo), 2);
        let matrix = create_complement_matrix(&vec![ludicolo, primal_groundon]);
        println!("{matrix:?}");
    }

    #[test]
    fn find_complements() {
        use BasicType::*;
        let poke = Pokemon {
            typing: (Water).into(),
            ability: None,
        };
        Pokemon::all()
            .into_iter()
            .map(|p| (p.clone(), resistance_complements(&poke, &p)))
            .unique()
            .filter(|(_, score)| *score >= 2)
            .for_each(|p| {
                println!("{:?}", p);
            });
    }

    #[test]
    fn find_complemented() {
        use BasicType::*;
        let poke = Pokemon {
            typing: (Grass, Ice).into(),
            ability: None,
        };
        // vec![Pokemon {
        //     typing: (Poison, Ghost).into(),
        //     ability: None,
        // }]
        Pokemon::all()
            .into_iter()
            .map(|p| (p.clone(), resistance_complements(&p, &poke)))
            .unique()
            .filter(|(_, score)| *score >= -50)
            .sorted_by(|(_, s1), (_, s2)| s1.cmp(s2))
            .for_each(|p| {
                println!("{:?}", p);
            });
    }

    #[test]
    fn find_best_connector_test() {
        use BasicType::*;
        let pool = Pokemon::all_type_combinations_and_abilities().collect::<Vec<_>>();
        let connections = resistance_connector(
            &Pokemon {
                typing: (Water).into(),
                ability: None,
            },
            &Pokemon {
                typing: (Ground, Flying).into(),
                ability: None,
            },
            &pool,
        )
        .into_iter()
        .for_each(|(p, s1, s2)| {
            println!("{p:?} {s1:?} {s2:?}");
        });
    }

    #[test]
    fn find_every_steel_complement() {
        use BasicType::*;
        Pokemon::all()
            .into_iter()
            .filter(|p| p.typing.contains(Steel))
            .unique()
            .for_each(|poke| {
                println!("{poke:?}");
                Pokemon::all()
                    .into_iter()
                    .unique()
                    .map(|p| (p.clone(), resistance_complements(&poke, &p)))
                    .max_set_by_key(|(_, s)| *s)
                    .into_iter()
                    .for_each(|(p, s)| {
                        println!("  {p:?} {s:?}");
                    });
            });
    }

    #[test]
    fn recursive_search_specific_type_complements() {
        use BasicType::*;
        let root = Pokemon::from((Steel, Flying));
        for _ in 0..1 {
            let best_complements = Pokemon::all()
                .into_iter()
                .unique()
                .map(|p| (p.clone(), resistance_complements(&root, &p)))
                .max_set_by_key(|(_, s)| *s);
            best_complements.iter().for_each(|(p, _)| {
                println!("{p:?} {:?}", resistance_complements(&p, &root));
            });
        }
    }
}
