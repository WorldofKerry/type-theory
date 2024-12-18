use std::{collections::HashMap, str::FromStr};

use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

use crate::typing::{
    combine_defense_charts, get_multitype_defense_chart, Ability, BasicType, Relationship, Type,
    TypeTrait,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Typing {
    Mono(BasicType),
    Dual(BasicType, BasicType),
}

impl Into<Typing> for BasicType {
    fn into(self) -> Typing {
        Typing::Mono(self)
    }
}

impl Into<Typing> for (BasicType, BasicType) {
    fn into(self) -> Typing {
        Typing::Dual(self.0, self.1)
    }
}

impl Typing {
    fn mono() -> impl Iterator<Item = Typing> {
        BasicType::iter().map(Typing::Mono)
    }
    fn dual() -> impl Iterator<Item = Typing> {
        BasicType::iter().flat_map(|t1| {
            BasicType::iter()
                .filter(move |t2| t1 != *t2)
                .map(move |t2| Typing::Dual(t1, t2))
        })
    }
    fn all() -> impl Iterator<Item = Typing> {
        Typing::mono().chain(Typing::dual())
    }
}

impl TypeTrait for Typing {
    fn defense(&self) -> Relationship {
        match self {
            Typing::Mono(t) => t.defense(),
            Typing::Dual(t1, t2) => combine_defense_charts(vec![t1.defense(), t2.defense()]),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pokemon {
    pub typing: Typing,
    pub ability: Option<Ability>,
}

impl Pokemon {
    pub fn all() -> Vec<Pokemon> {
        // dexnum,name,generation,type1,type2,species,height,weight,ability1,ability2,hidden_ability,hp,attack,defense,sp_atk,sp_def,speed,total,ev_yield,catch_rate,base_friendship,base_exp,growth_rate,egg_group1,egg_group2,percent_male,percent_female,egg_cycles,special_group
        let file = "data/pokemon_data.csv"; // https://www.kaggle.com/datasets/guavocado/pokemon-stats-1025-pokemons

        let mut rdr = csv::Reader::from_path(file).unwrap();
        rdr.records()
            .flat_map(|r| {
                let record = r.unwrap();
                let typing = match record.get(4).unwrap() {
                    "" => Typing::Mono(BasicType::from_str(record.get(3).unwrap()).unwrap()),
                    t => Typing::Dual(
                        BasicType::from_str(record.get(3).unwrap()).unwrap(),
                        BasicType::from_str(t).unwrap(),
                    ),
                };
                let abilities: Vec<Option<Ability>> = vec![
                    record.get(8).unwrap(),
                    record.get(9).unwrap(),
                    record.get(10).unwrap(),
                ].into_iter().map(|a| match a {
                    "" => None,
                    a => match Ability::from_str(a) {
                        Ok(a) => Some(a),
                        Err(_) => None,
                    },
                }).collect();
                abilities.into_iter().map(move |a| Pokemon {
                    typing: typing.clone(),
                    ability: a,
                })
            })
            .collect()
    }

    pub fn all_type_combinations() -> impl Iterator<Item = Pokemon> {
        // All monotype/dualtype and ability combinations
        Typing::all()
            .flat_map(|t| {
                Ability::iter().map(move |a| Pokemon {
                    typing: t.clone(),
                    ability: Some(a),
                })
            })
            .chain(Typing::all().map(|t| Pokemon {
                typing: t,
                ability: None,
            }))
    }

    pub fn all_type_combinations_and_abilities() -> impl Iterator<Item = Pokemon> {
        Typing::all().map(|t| Pokemon {
            typing: t,
            ability: None,
        })
    }

    pub fn random(pool: &Vec<Pokemon>) -> Pokemon {
        let mut rng = rand::thread_rng();
        pool.choose(&mut rng).unwrap().clone()
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

    #[test]
    fn test_get_defense() {
        let duraludon = Pokemon {
            typing: Typing::Dual(BasicType::Dragon, BasicType::Steel),
            ability: None,
        };
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
    fn all_pokemon() {
        let all = Pokemon::all();
        println!("{:?}", all[0..500].to_vec());
    }
}
