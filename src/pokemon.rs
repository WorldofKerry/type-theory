use std::collections::HashMap;

use crate::typing::{combine_defense_charts, get_multitype_defense_chart, Ability, BasicType, Relationship, Type, TypeTrait};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Typing {
    Mono(Type),
    Dual(Type, Type),
}

impl TypeTrait for Typing {
    fn get_defense(&self) -> Relationship {
        let all_types: Vec<&Type> = match self {
            Typing::Mono(t) => vec![t],
            Typing::Dual(t1, t2) => vec![t1, t2],
        };
        get_multitype_defense_chart(all_types.into_iter())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Pokemon {
    typing: Typing,
    ability: Option<Ability>,
}

impl TypeTrait for Pokemon {
    fn get_defense(&self) -> Relationship {
        match self.ability {
            Some(a) => combine_defense_charts(vec![self.typing.get_defense(), a.get_defense()]),
            None => self.typing.get_defense(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_defense() {
        let duraludon = Pokemon {
            typing: Typing::Dual(Type::Basic(BasicType::Dragon), Type::Basic(BasicType::Steel)),
            ability: None,
        };
        println!("{:?}", BasicType::Dragon.get_defense());
        let defense_chart = duraludon.get_defense();
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