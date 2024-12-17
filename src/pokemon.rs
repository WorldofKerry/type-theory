use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::typing::{combine_defense_charts, get_multitype_defense_chart, Ability, BasicType, Relationship, Type, TypeTrait};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Typing {
    Mono(BasicType),
    Dual(BasicType, BasicType),
}

impl Typing {
    fn mono() -> impl Iterator<Item = Typing> {
        BasicType::iter().map(Typing::Mono)
    }
    fn dual() -> impl Iterator<Item = Typing> {
        BasicType::iter().flat_map(|t1| BasicType::iter().map(move |t2| Typing::Dual(t1, t2)))
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
struct Pokemon {
    typing: Typing,
    ability: Option<Ability>,
}

impl Pokemon {
    fn all() -> impl Iterator<Item = Pokemon> {
        // All monotype/dualtype and ability combinations
        Typing::all().flat_map(|t| {
            Ability::iter().map(move |a| Pokemon { typing: t.clone(), ability: Some(a) })
        }).chain(Typing::all().map(|t| Pokemon { typing: t, ability: None }))
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