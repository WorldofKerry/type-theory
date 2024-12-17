use std::collections::HashMap;

use crate::typing::{combine_defense_charts, get_multitype_defense_chart, Ability, BasicType, Type, TypeTrait};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Typing {
    Mono(Type),
    Dual(Type, Type),
}

impl TypeTrait for Typing {
    fn get_defense(&self) -> HashMap<BasicType, f32> {
        let all_types: Vec<&Type> = match self {
            Typing::Mono(t) => vec![t],
            Typing::Dual(t1, t2) => vec![t1, t2],
        };
        get_multitype_defense_chart(all_types)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Pokemon {
    typing: Typing,
    ability: Option<Ability>,
}

impl TypeTrait for Pokemon {
    fn get_defense(&self) -> HashMap<BasicType, f32> {
        let mut defense_chart = self.typing.get_defense();
        if let Some(ability) = self.ability {
            let ability_chart = ability.get_defense();
            defense_chart = combine_defense_charts(vec![defense_chart, ability_chart]);
        }
        defense_chart
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
        assert_eq!(defense_chart.get(&BasicType::Normal), Some(&0.5));
        assert_eq!(defense_chart.get(&BasicType::Water), Some(&0.5));
        assert_eq!(defense_chart.get(&BasicType::Electric), Some(&0.5));
        assert_eq!(defense_chart.get(&BasicType::Grass), Some(&0.25));
        assert_eq!(defense_chart.get(&BasicType::Fighting), Some(&2.0));
        assert_eq!(defense_chart.get(&BasicType::Poison), Some(&0.0));
        assert_eq!(defense_chart.get(&BasicType::Ground), Some(&2.0));
        assert_eq!(defense_chart.get(&BasicType::Flying), Some(&0.5));
        assert_eq!(defense_chart.get(&BasicType::Psychic), Some(&0.5));
        assert_eq!(defense_chart.get(&BasicType::Bug), Some(&0.5));
        assert_eq!(defense_chart.get(&BasicType::Rock), Some(&0.5));
        assert_eq!(defense_chart.get(&BasicType::Steel), Some(&0.5));
    }
}