use crate::typing::{combine_defense_charts, Ability, BasicType, Relationship, TypeTrait};
use itertools::Itertools;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, str::FromStr, sync::OnceLock};
use strum::IntoEnumIterator;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Deserialize, Serialize)]
pub struct Typing(BTreeSet<BasicType>);

impl Typing {
    pub fn iter(&self) -> impl Iterator<Item = &BasicType> {
        self.0.iter()
    }
}

impl Typing {
    pub fn contains(&self, t: BasicType) -> bool {
        self.0.contains(&t)
    }
}

impl From<BasicType> for Typing {
    fn from(t: BasicType) -> Typing {
        Typing(BTreeSet::from_iter(vec![t]))
    }
}

impl From<(BasicType, BasicType)> for Typing {
    fn from(t: (BasicType, BasicType)) -> Typing {
        Typing(BTreeSet::from_iter(vec![t.0, t.1]))
    }
}

impl Typing {
    fn mono() -> impl Iterator<Item = Typing> {
        BasicType::iter().map(Typing::from)
    }
    fn dual() -> impl Iterator<Item = Typing> {
        BasicType::iter()
            .combinations(2)
            .map(|c| Typing::from((c[0], c[1])))
    }
    fn all() -> impl Iterator<Item = Typing> {
        Typing::mono().chain(Typing::dual())
    }
}

impl TypeTrait for Typing {
    fn defense(&self) -> Relationship {
        match self.0.iter().collect::<Vec<_>>().as_slice() {
            [t] => t.defense(),
            [t1, t2] => combine_defense_charts(vec![t1.defense(), t2.defense()]),
            _ => panic!("Invalid typing"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Move {
    pub name: String,
    pub typing: BasicType,
    pub power: Option<u32>,
}

impl Move {
    pub fn all() -> Vec<Move> {
        // name,id,accuracy,pp,power,priority,type,generation,short_descripton,damage_class
        let file = "data/metadata_pokemon_moves.csv";
        let mut rdr = csv::Reader::from_path(file).unwrap();
        rdr.records()
            .filter_map(|r| {
                let record = r.unwrap();
                let name = record.get(0).unwrap().to_string();
                let typing = match BasicType::from_str(record.get(6).unwrap()) {
                    Ok(t) => t,
                    Err(_) => BasicType::Normal,
                };
                let power = match record.get(4).unwrap() {
                    "" => None,
                    p => Some(p.parse::<f32>().unwrap() as u32),
                };
                Some(Move {
                    name,
                    typing,
                    power,
                })
            })
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Pokemon {
    pub species: String,
    pub typing: Typing,
    pub ability: Option<Ability>,
    pub moves: Vec<Move>,
}

impl From<BasicType> for Pokemon {
    fn from(t: BasicType) -> Pokemon {
        Pokemon {
            species: "".into(),
            typing: t.into(),
            ability: None,
            moves: vec![],
        }
    }
}

impl From<(BasicType, BasicType)> for Pokemon {
    fn from(t: (BasicType, BasicType)) -> Pokemon {
        Pokemon {
            species: "".into(),
            typing: t.into(),
            ability: None,
            moves: vec![],
        }
    }
}

pub trait PokemonIteratorHelper: Iterator<Item = Pokemon> {
    fn unique_by_type_ability(self) -> impl Iterator<Item = Pokemon>
    where
        Self: Sized,
    {
        self.unique_by(|p| (p.typing.clone(), p.ability))
    }
}

impl<I> PokemonIteratorHelper for I where I: Iterator<Item = Pokemon> + ?Sized {}

impl Pokemon {
    pub fn all() -> &'static Vec<Pokemon> {
        static CELL: OnceLock<Vec<Pokemon>> = OnceLock::new();
        CELL.get_or_init(|| {
            // dexnum,name,generation,type1,type2,species,height,weight,ability1,ability2,hidden_ability,hp,attack,defense,sp_atk,sp_def,speed,total,ev_yield,catch_rate,base_friendship,base_exp,growth_rate,egg_group1,egg_group2,percent_male,percent_female,egg_cycles,special_group
            let file = "data/pokemon_data_gen5.csv";
            #[cfg(feature = "gen6")]
            let file = "data/pokemon_data_gen6+.csv";
            csv::Reader::from_path(file)
                .unwrap()
                .records()
                .flat_map(|r| {
                    let record = r.unwrap();
                    let name = record.get(1).unwrap().to_string();
                    let typing = match record.get(4).unwrap() {
                        "" => Typing::from(BasicType::from_str(record.get(3).unwrap()).unwrap()),
                        t => Typing::from((
                            BasicType::from_str(record.get(3).unwrap()).unwrap(),
                            BasicType::from_str(t).unwrap(),
                        )),
                    };
                    let abilities: Vec<Option<Ability>> = vec![
                        record.get(8).unwrap(),
                        record.get(9).unwrap(),
                        record.get(10).unwrap(),
                    ]
                    .into_iter()
                    .map(|a| match a {
                        "" => None,
                        a => match Ability::from_str(a) {
                            Ok(a) => Some(a),
                            Err(_) => None,
                        },
                    })
                    .collect();
                    abilities.into_iter().map(move |a| Pokemon {
                        species: name.clone(),
                        typing: typing.clone(),
                        ability: a,
                        moves: vec![],
                    })
                })
                .chain(
                    // Rotom forms
                    vec![
                        (BasicType::Electric, BasicType::Fire),
                        (BasicType::Electric, BasicType::Water),
                        (BasicType::Electric, BasicType::Ice),
                        (BasicType::Electric, BasicType::Flying),
                        (BasicType::Electric, BasicType::Grass),
                    ]
                    .into_iter()
                    .map(|t| Pokemon {
                        species: "Rotom".into(),
                        typing: t.into(),
                        ability: Some(Ability::Levitate),
                        moves: vec![],
                    }),
                )
                .collect()
        })
    }

    // All pokemon, unique by typing and ability
    pub fn all_unique_type_chart() -> &'static Vec<Pokemon> {
        static CELL: OnceLock<Vec<Pokemon>> = OnceLock::new();
        CELL.get_or_init(|| {
            Pokemon::all()
                .clone()
                .into_iter()
                .unique_by_type_ability()
                .collect()
        })
    }

    pub fn all_type_combinations_and_abilities() -> impl Iterator<Item = Pokemon> {
        // All monotype/dualtype and ability combinations
        Typing::all()
            .flat_map(|t| {
                Ability::iter().map(move |a| Pokemon {
                    species: "".into(),
                    typing: t.clone(),
                    ability: Some(a),
                    moves: vec![],
                })
            })
            .chain(Typing::all().map(|t| Pokemon {
                species: "".into(),
                typing: t,
                ability: None,
                moves: vec![],
            }))
    }

    pub fn all_type_combinations() -> impl Iterator<Item = Pokemon> {
        Typing::all().map(|t| Pokemon {
            species: "".into(),
            typing: t,
            ability: None,
            moves: vec![],
        })
    }

    pub fn random(pool: &Vec<Pokemon>) -> Pokemon {
        let mut rng = rand::thread_rng();
        pool.choose(&mut rng).unwrap().clone()
    }

    pub fn random_team(pool: &Vec<Pokemon>, size: usize) -> Vec<Pokemon> {
        pool.choose_multiple(&mut rand::thread_rng(), size)
            .cloned()
            .sorted()
            .collect()
    }

    pub fn random_fill(team: &Vec<Pokemon>, pool: &Vec<Pokemon>, size: usize) -> Vec<Pokemon> {
        let missing = size - team.len();
        let mut new_team = team.clone();
        new_team.extend(Pokemon::random_team(pool, missing));
        new_team.into_iter().sorted().collect()
    }
}

impl TypeTrait for Pokemon {
    fn defense(&self) -> Relationship {
        match self.ability {
            Some(a) => combine_defense_charts(vec![self.typing.defense(), a.defense()]),
            None => self.typing.defense(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hash::{Hash, Hasher};

    #[test]
    fn test_pokemon_all() {
        let all_pokemon = Pokemon::all();
        // Note that pokemon with multiple abilities that affect the type chart are split into multiple entries
        let expected_count = 1952;
        #[cfg(feature = "gen6")]
        let expected_count = 3080;
        assert_eq!(all_pokemon.len(), expected_count);
    }

    #[test]
    fn test_get_defense() {
        let duraludon = Pokemon::from((BasicType::Dragon, BasicType::Steel));
        println!("{:?}", BasicType::Dragon.defense());
        let defense_chart = duraludon.defense();
        assert_eq!(defense_chart.get(BasicType::Normal), 0.5);
        assert_eq!(defense_chart.get(BasicType::Water), 0.5);
        assert_eq!(defense_chart.get(BasicType::Electric), 0.5);
        assert_eq!(defense_chart.get(BasicType::Grass), 0.25);
        assert_eq!(defense_chart.get(BasicType::Fighting), 2.0);
        assert_eq!(defense_chart.get(BasicType::Poison), 0.0);
        assert_eq!(defense_chart.get(BasicType::Ground), 2.0);
        assert_eq!(defense_chart.get(BasicType::Flying), 0.5);
        assert_eq!(defense_chart.get(BasicType::Psychic), 0.5);
        assert_eq!(defense_chart.get(BasicType::Bug), 0.5);
        assert_eq!(defense_chart.get(BasicType::Rock), 0.5);
        assert_eq!(defense_chart.get(BasicType::Steel), 0.5);
    }

    #[test]
    fn equality_test() {
        use BasicType::*;
        let poke1 = Pokemon::from((Steel, Flying));
        let poke2 = Pokemon::from((Flying, Steel));
        assert_eq!(poke1, poke2);
        let mut hasher1 = std::hash::DefaultHasher::new();
        let mut hasher2 = std::hash::DefaultHasher::new();
        poke1.hash(&mut hasher1);
        poke2.hash(&mut hasher2);
        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_all_moves() {
        let moves = Move::all();
        assert_eq!(moves[0].power, Some(40));

        let expected_move_count = 808;
        #[cfg(feature = "gen6")]
        let expected_move_count = 808;

        assert_eq!(moves.len(), expected_move_count);
    }
}
