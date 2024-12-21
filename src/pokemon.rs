use std::{collections::BTreeSet, str::FromStr};

use itertools::Itertools;
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

use crate::typing::{combine_defense_charts, Ability, BasicType, Relationship, TypeTrait};

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
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
        BasicType::iter().flat_map(|t1| {
            BasicType::iter()
                .filter(move |t2| t1 != *t2)
                .map(move |t2| Typing::from((t1, t2)))
        })
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

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Move {
    pub name: String,
    pub typing: Typing,
    pub power: Option<u32>,
}

impl Move {
    pub fn all() -> Vec<Move> {
        // name,id,accuracy,pp,power,priority,type,generation,short_descripton,damage_class
        let file = "data/metadata_pokemon_moves.csv";
        let mut rdr = csv::Reader::from_path(file).unwrap();
        rdr.records()
            .map(|r| {
                let record = r.unwrap();
                let name = record.get(0).unwrap().to_string();
                let typing = BasicType::from_str(record.get(6).unwrap()).unwrap().into();
                let power = match record.get(4).unwrap() {
                    "" => None,
                    p => Some(p.parse::<f32>().unwrap() as u32),
                };
                Move {
                    name,
                    typing,
                    power,
                }
            })
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

impl Pokemon {
    pub fn from_pkhex_dump(file: &str) -> Vec<Pokemon> {
        // "Position","Nickname","Species","Nature","Gender","ESV","HP_Type","Ability","Move1","Move2","Move3","Move4","HeldItem","HP","ATK","DEF","SPA","SPD","SPE","MetLoc","EggLoc","Ball","OT","Version","OTLang","Legal","EC","PID","IV_HP","IV_ATK","IV_DEF","IV_SPA","IV_SPD","IV_SPE","EXP","Level","EV_HP","EV_ATK","EV_DEF","EV_SPA","EV_SPD","EV_SPE","Cool","Beauty","Cute","Smart","Tough","Sheen","NotOT","AbilityNum","GenderFlag","Form","PokerusStrain","PokerusDays","MetLevel","OriginalTrainerGender","FatefulEncounter","IsEgg","IsNicknamed","IsShiny","TID16","SID16","TSV","Move1_PP","Move2_PP","Move3_PP","Move4_PP","Move1_PPUp","Move2_PPUp","Move3_PPUp","Move4_PPUp","Relearn1","Relearn2","Relearn3","Relearn4","Checksum","Friendship","EggYear","EggMonth","EggDay","MetYear","MetMonth","MetDay"
        let all_pokemon = Pokemon::all();
        let all_moves = Move::all();
        let mut rdr = csv::Reader::from_path(file).unwrap();
        rdr.records()
            .map(|r| {
                let record = r.unwrap();
                let species = record.get(2).unwrap().to_string();
                let ability = match record.get(7).unwrap() {
                    "" => None,
                    a => match Ability::from_str(a) {
                        Ok(a) => Some(a),
                        Err(_) => None,
                    },
                };
                let typing = all_pokemon
                    .iter()
                    .find(|p| p.species == species)
                    .unwrap()
                    .typing
                    .clone();
                let moves = vec![
                    record.get(8).unwrap(),
                    record.get(9).unwrap(),
                    record.get(10).unwrap(),
                    record.get(11).unwrap(),
                ]
                .into_iter()
                .flat_map(|m| {
                    let name = m.to_string();
                    let name = name.replace("-", " ");
                    if name == "(None)" {
                        return vec![];
                    } else {
                        return vec![all_moves
                            .iter()
                            .find(|m| m.name.to_lowercase() == name.to_lowercase())
                            .expect(&format!("{name:?}"))
                            .clone()];
                    }
                })
                .collect();
                Pokemon {
                    species,
                    typing,
                    ability,
                    moves,
                }
            })
            .collect()
    }

    pub fn all() -> Vec<Pokemon> {
        // dexnum,name,generation,type1,type2,species,height,weight,ability1,ability2,hidden_ability,hp,attack,defense,sp_atk,sp_def,speed,total,ev_yield,catch_rate,base_friendship,base_exp,growth_rate,egg_group1,egg_group2,percent_male,percent_female,egg_cycles,special_group
        let file = "data/pokemon_data.csv";

        let mut rdr = csv::Reader::from_path(file).unwrap();
        rdr.records()
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
            .collect()
    }

    pub fn all_type_combinations() -> impl Iterator<Item = Pokemon> {
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

    pub fn all_type_combinations_and_abilities() -> impl Iterator<Item = Pokemon> {
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
            .collect()
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
        assert!(all_pokemon.len() == 3075);
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
    fn test_from_pkhex_dump() {
        let file = "data/Box Data Dump.csv";
        let team = Pokemon::from_pkhex_dump(file);
        assert!(team.len() > 10);
    }

    #[test]
    fn test_all_moves() {
        let moves = Move::all();
        assert_eq!(moves[0].power, Some(40));
        assert!(moves.len() == 808);
    }
}
