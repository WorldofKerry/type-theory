use std::collections::HashMap;

use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

use crate::typing::{combine_defense_charts, get_multitype_defense_chart, Ability, BasicType, Relationship, Type, TypeTrait};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Typing {
    Mono(BasicType),
    Dual(BasicType, BasicType),
}

impl Typing {
    fn mono() -> impl Iterator<Item = Typing> {
        BasicType::iter().map(Typing::Mono)
    }
    fn dual() -> impl Iterator<Item = Typing> {
        BasicType::iter().flat_map(|t1| BasicType::iter().filter(move |t2| t1 != *t2).map(move |t2| Typing::Dual(t1, t2)))
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
    pub fn all() -> impl Iterator<Item = Pokemon> {
        // All monotype/dualtype and ability combinations
        Typing::all().flat_map(|t| Ability::iter().map(move |a| Pokemon { typing: t.clone(), ability: Some(a) }))
            .chain(Typing::all().map(|t| Pokemon { typing: t, ability: None }))
    }

    pub fn all_no_abilities() -> impl Iterator<Item = Pokemon> {
        Typing::all().map(|t| Pokemon { typing: t, ability: None })
    }

    pub fn random(pool: &Vec<Pokemon>) -> Pokemon {
        let mut rng = rand::thread_rng();
        pool.choose(&mut rng).unwrap().clone()
    }

    pub fn find_resistance_complements(&self, pool: impl Iterator<Item = Pokemon>) -> Vec<Pokemon> {
        let def = self.defense();
        pool.filter(move |p| {
            let compl_def = p.defense();
            // Not any weakness not resisted
            !def.iter().any(|(t, r)| *r > 1.0 && compl_def.get(*t) >= 1.0)
        }).collect()
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
}