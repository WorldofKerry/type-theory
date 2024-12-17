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
    ability: Ability,
}

impl TypeTrait for Pokemon {
    fn get_defense(&self) -> HashMap<BasicType, f32> {
        combine_defense_charts(vec![self.typing.get_defense(), self.ability.get_defense()])
    }
}