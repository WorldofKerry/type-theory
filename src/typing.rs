use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BasicType {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Ability {
    Levitate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Basic(BasicType),
    Ability(Ability),
}

impl TypeTrait for BasicType {
    fn get_defense(&self) -> HashMap<BasicType, f32> {
        get_defense_chart().get(&Type::Basic(*self)).unwrap().clone()
    }
}

impl TypeTrait for Ability {
    fn get_defense(&self) -> HashMap<BasicType, f32> {
        get_defense_chart().get(&Type::Ability(*self)).unwrap().clone()
    }
}

pub trait TypeTrait {
    fn get_defense(&self) -> HashMap<BasicType, f32>;
}

impl TypeTrait for Type {
    fn get_defense(&self) -> HashMap<BasicType, f32> {
        match self {
            Type::Basic(basic_type) => basic_type.get_defense(),
            Type::Ability(ability) => ability.get_defense(),
        }
    }
}

pub fn combine_defense_charts(charts: Vec<HashMap<BasicType, f32>>) -> HashMap<BasicType, f32> {
    let mut defense_chart = HashMap::new();
    for chart in charts {
        for (basic_type, multiplier) in chart {
            let entry = defense_chart.entry(basic_type).or_insert(1.0);
            *entry *= multiplier;
        }
    }
    defense_chart
}

pub fn get_multitype_defense_chart(types: Vec<&Type>) -> HashMap<BasicType, f32> {
    let mut defense_chart = HashMap::new();
    for t in types {
        let chart = t.get_defense();
        defense_chart = combine_defense_charts(vec![defense_chart, chart]);
    }
    defense_chart
}

fn get_defense_chart() -> HashMap<Type, HashMap<BasicType, f32>> {
    HashMap::from([
        (
            Type::Basic(BasicType::Normal),
            HashMap::from([
                ((BasicType::Fighting), 2.0),
                ((BasicType::Ghost), 0.0),
            ]),
        ),
        (
            Type::Basic(BasicType::Fire),
            HashMap::from([
                ((BasicType::Fire), 0.5),
                ((BasicType::Water), 2.0),
                ((BasicType::Grass), 0.5),
                ((BasicType::Ice), 0.5),
                ((BasicType::Ground), 2.0),
                ((BasicType::Bug), 0.5),
                ((BasicType::Rock), 2.0),
                ((BasicType::Steel), 0.5),
                ((BasicType::Fairy), 0.5),
            ]),
        ),
        (
            Type::Basic(BasicType::Water),
            HashMap::from([
                ((BasicType::Fire), 0.5),
                ((BasicType::Water), 0.5),
                ((BasicType::Electric), 2.0),
                ((BasicType::Grass), 2.0),
                ((BasicType::Ice), 0.5),
                ((BasicType::Steel), 0.5),
            ]),
        ),
        (
            Type::Basic(BasicType::Electric),
            HashMap::from([
                ((BasicType::Electric), 0.5),
                ((BasicType::Ground), 2.0),
                ((BasicType::Flying), 0.5),
                ((BasicType::Steel), 0.5),
            ]),
        ),
        (
            Type::Basic(BasicType::Grass),
            HashMap::from([
                ((BasicType::Fire), 2.0),
                ((BasicType::Water), 0.5),
                ((BasicType::Electric), 0.5),
                ((BasicType::Grass), 0.5),
                ((BasicType::Ice), 2.0),
                ((BasicType::Poison), 2.0),
                ((BasicType::Ground), 0.5),
                ((BasicType::Flying), 2.0),
                ((BasicType::Bug), 2.0),
            ]),
        ),
        (
            Type::Basic(BasicType::Ice),
            HashMap::from([
                ((BasicType::Fire), 2.0),
                ((BasicType::Ice), 0.5),
                ((BasicType::Fighting), 2.0),
                ((BasicType::Rock), 2.0),
                ((BasicType::Steel), 2.0),
            ]),
        ),
        (
            Type::Basic(BasicType::Fighting),
            HashMap::from([
                ((BasicType::Flying), 2.0),
                ((BasicType::Psychic), 2.0),
                ((BasicType::Bug), 0.5),
                ((BasicType::Rock), 0.5),
                ((BasicType::Dark), 0.5),
                ((BasicType::Fairy), 2.0),
            ]),
        ),
        (
            Type::Basic(BasicType::Poison),
            HashMap::from([
                ((BasicType::Grass), 0.5),
                ((BasicType::Fighting), 0.5),
                ((BasicType::Poison), 0.5),
                ((BasicType::Ground), 2.0),
                ((BasicType::Psychic), 2.0),
                ((BasicType::Bug), 0.5),
                ((BasicType::Fairy), 0.5),
            ]),
        ),
        (
            Type::Basic(BasicType::Ground),
            HashMap::from([
                ((BasicType::Water), 2.0),
                ((BasicType::Electric), 0.0),
                ((BasicType::Grass), 2.0),
                ((BasicType::Ice), 2.0),
                ((BasicType::Poison), 0.5),
                ((BasicType::Rock), 0.5),
            ]),
        ),
        (
            Type::Basic(BasicType::Flying),
            HashMap::from([
                ((BasicType::Electric), 2.0),
                ((BasicType::Grass), 0.5),
                ((BasicType::Ice), 2.0),
                ((BasicType::Fighting), 0.5),
                ((BasicType::Ground), 0.0),
                ((BasicType::Bug), 0.5),
                ((BasicType::Rock), 2.0),
            ]),
        ),
        (
            Type::Basic(BasicType::Psychic),
            HashMap::from([
                ((BasicType::Fighting), 0.5),
                ((BasicType::Psychic), 0.5),
                ((BasicType::Bug), 2.0),
                ((BasicType::Ghost), 2.0),
                ((BasicType::Dark), 2.0),
            ]),
        ),
        (
            Type::Basic(BasicType::Bug),
            HashMap::from([
                ((BasicType::Fire), 2.0),
                ((BasicType::Grass), 0.5),
                ((BasicType::Fighting), 0.5),
                ((BasicType::Ground), 0.5),
                ((BasicType::Flying), 2.0),
                ((BasicType::Rock), 2.0),
            ]),
        ),
        (
            Type::Basic(BasicType::Rock),
            HashMap::from([
                ((BasicType::Normal), 0.5),
                ((BasicType::Fire), 0.5),
                ((BasicType::Water), 2.0),
                ((BasicType::Grass), 2.0),
                ((BasicType::Fighting), 2.0),
                ((BasicType::Poison), 0.5),
                ((BasicType::Ground), 2.0),
                ((BasicType::Flying), 0.5),
            ]),
        ),
        (
            Type::Basic(BasicType::Ghost),
            HashMap::from([
                ((BasicType::Normal), 0.0),
                ((BasicType::Psychic), 0.0),
                ((BasicType::Poison), 0.5),
                ((BasicType::Bug), 0.5),
                ((BasicType::Ghost), 2.0),
                ((BasicType::Dark), 2.0),
            ]),
        ),
        (
            Type::Basic(BasicType::Dragon),
            HashMap::from([
                ((BasicType::Fire), 0.5),
                ((BasicType::Water), 0.5),
                ((BasicType::Electric), 0.5),
                ((BasicType::Grass), 0.5),
                ((BasicType::Ice), 2.0),
                ((BasicType::Dragon), 2.0),
                ((BasicType::Fairy), 2.0),
            ]),
        ),
        (
            Type::Basic(BasicType::Dark),
            HashMap::from([
                ((BasicType::Fighting), 2.0),
                ((BasicType::Psychic), 0.0),
                ((BasicType::Bug), 2.0),
                ((BasicType::Ghost), 0.5),
                ((BasicType::Dark), 0.5),
                ((BasicType::Fairy), 2.0),
            ]),
        ),
        (
            Type::Basic(BasicType::Steel),
            HashMap::from([
                ((BasicType::Normal), 0.5),
                ((BasicType::Fire), 2.0),
                ((BasicType::Grass), 0.5),
                ((BasicType::Ice), 0.5),
                ((BasicType::Fighting), 2.0),
                ((BasicType::Poison), 0.0),
                ((BasicType::Ground), 2.0),
                ((BasicType::Flying), 0.5),
                ((BasicType::Psychic), 0.5),
                ((BasicType::Bug), 0.5),
                ((BasicType::Rock), 0.5),
                ((BasicType::Dragon), 0.5),
                ((BasicType::Steel), 0.5),
                ((BasicType::Fairy), 0.5),
            ]),
        ),
        (
            Type::Basic(BasicType::Fairy),
            HashMap::from([
                ((BasicType::Fighting), 0.5),
                ((BasicType::Poison), 2.0),
                ((BasicType::Bug), 0.5),
                ((BasicType::Dragon), 0.0),
                ((BasicType::Dark), 0.5),
                ((BasicType::Steel), 2.0),
            ]),
        ),
        (
            Type::Ability(Ability::Levitate),
            HashMap::from([
                ((BasicType::Ground), 0.0),
            ]),
        )
    ])
}