use std::{collections::HashMap, ops::Add};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum BasicType {
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Ability {
    Levitate,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Type {
    Basic(BasicType),
    Ability(Ability),
}

fn get_defense_chart() -> HashMap<Type, HashMap<Type, f32>> {
    HashMap::from([
        (
            Type::Basic(BasicType::Normal),
            HashMap::from([
                (Type::Basic(BasicType::Fighting), 2.0),
                (Type::Basic(BasicType::Ghost), 0.0),
            ]),
        ),
        (
            Type::Basic(BasicType::Fire),
            HashMap::from([
                (Type::Basic(BasicType::Fire), 0.5),
                (Type::Basic(BasicType::Water), 2.0),
                (Type::Basic(BasicType::Grass), 0.5),
                (Type::Basic(BasicType::Ice), 0.5),
                (Type::Basic(BasicType::Ground), 2.0),
                (Type::Basic(BasicType::Bug), 0.5),
                (Type::Basic(BasicType::Rock), 2.0),
                (Type::Basic(BasicType::Steel), 0.5),
                (Type::Basic(BasicType::Fairy), 0.5),
            ]),
        ),
        (
            Type::Basic(BasicType::Water),
            HashMap::from([
                (Type::Basic(BasicType::Fire), 0.5),
                (Type::Basic(BasicType::Water), 0.5),
                (Type::Basic(BasicType::Electric), 2.0),
                (Type::Basic(BasicType::Grass), 2.0),
                (Type::Basic(BasicType::Ice), 0.5),
                (Type::Basic(BasicType::Steel), 0.5),
            ]),
        ),
        (
            Type::Basic(BasicType::Electric),
            HashMap::from([
                (Type::Basic(BasicType::Electric), 0.5),
                (Type::Basic(BasicType::Ground), 2.0),
                (Type::Basic(BasicType::Flying), 0.5),
                (Type::Basic(BasicType::Steel), 0.5),
            ]),
        ),
        (
            Type::Basic(BasicType::Grass),
            HashMap::from([
                (Type::Basic(BasicType::Fire), 2.0),
                (Type::Basic(BasicType::Water), 0.5),
                (Type::Basic(BasicType::Electric), 0.5),
                (Type::Basic(BasicType::Grass), 0.5),
                (Type::Basic(BasicType::Ice), 2.0),
                (Type::Basic(BasicType::Poison), 2.0),
                (Type::Basic(BasicType::Ground), 0.5),
                (Type::Basic(BasicType::Flying), 2.0),
                (Type::Basic(BasicType::Bug), 2.0),
            ]),
        ),
        (
            Type::Basic(BasicType::Ice),
            HashMap::from([
                (Type::Basic(BasicType::Fire), 2.0),
                (Type::Basic(BasicType::Ice), 0.5),
                (Type::Basic(BasicType::Fighting), 2.0),
                (Type::Basic(BasicType::Rock), 2.0),
                (Type::Basic(BasicType::Steel), 2.0),
            ]),
        ),
        (
            Type::Basic(BasicType::Fighting),
            HashMap::from([
                (Type::Basic(BasicType::Flying), 2.0),
                (Type::Basic(BasicType::Psychic), 2.0),
                (Type::Basic(BasicType::Bug), 0.5),
                (Type::Basic(BasicType::Rock), 0.5),
                (Type::Basic(BasicType::Dark), 0.5),
                (Type::Basic(BasicType::Fairy), 2.0),
            ]),
        ),
        (
            Type::Basic(BasicType::Poison),
            HashMap::from([
                (Type::Basic(BasicType::Grass), 0.5),
                (Type::Basic(BasicType::Fighting), 0.5),
                (Type::Basic(BasicType::Poison), 0.5),
                (Type::Basic(BasicType::Ground), 2.0),
                (Type::Basic(BasicType::Psychic), 2.0),
                (Type::Basic(BasicType::Bug), 0.5),
                (Type::Basic(BasicType::Fairy), 0.5),
            ]),
        ),
        (
            Type::Basic(BasicType::Ground),
            HashMap::from([
                (Type::Basic(BasicType::Water), 2.0),
                (Type::Basic(BasicType::Electric), 0.0),
                (Type::Basic(BasicType::Grass), 2.0),
                (Type::Basic(BasicType::Ice), 2.0),
                (Type::Basic(BasicType::Poison), 0.5),
                (Type::Basic(BasicType::Rock), 0.5),
            ]),
        ),
        (
            Type::Basic(BasicType::Flying),
            HashMap::from([
                (Type::Basic(BasicType::Electric), 2.0),
                (Type::Basic(BasicType::Grass), 0.5),
                (Type::Basic(BasicType::Ice), 2.0),
                (Type::Basic(BasicType::Fighting), 0.5),
                (Type::Basic(BasicType::Ground), 0.0),
                (Type::Basic(BasicType::Bug), 0.5),
                (Type::Basic(BasicType::Rock), 2.0),
            ]),
        ),
        (
            Type::Basic(BasicType::Psychic),
            HashMap::from([
                (Type::Basic(BasicType::Fighting), 0.5),
                (Type::Basic(BasicType::Psychic), 0.5),
                (Type::Basic(BasicType::Bug), 2.0),
                (Type::Basic(BasicType::Ghost), 2.0),
                (Type::Basic(BasicType::Dark), 2.0),
            ]),
        ),
        (
            Type::Basic(BasicType::Bug),
            HashMap::from([
                (Type::Basic(BasicType::Fire), 2.0),
                (Type::Basic(BasicType::Grass), 0.5),
                (Type::Basic(BasicType::Fighting), 0.5),
                (Type::Basic(BasicType::Ground), 0.5),
                (Type::Basic(BasicType::Flying), 2.0),
                (Type::Basic(BasicType::Rock), 2.0),
            ]),
        ),
        (
            Type::Basic(BasicType::Rock),
            HashMap::from([
                (Type::Basic(BasicType::Normal), 0.5),
                (Type::Basic(BasicType::Fire), 0.5),
                (Type::Basic(BasicType::Water), 2.0),
                (Type::Basic(BasicType::Grass), 2.0),
                (Type::Basic(BasicType::Fighting), 2.0),
                (Type::Basic(BasicType::Poison), 0.5),
                (Type::Basic(BasicType::Ground), 2.0),
                (Type::Basic(BasicType::Flying), 0.5),
            ]),
        ),
        (
            Type::Basic(BasicType::Ghost),
            HashMap::from([
                (Type::Basic(BasicType::Normal), 0.0),
                (Type::Basic(BasicType::Psychic), 0.0),
                (Type::Basic(BasicType::Poison), 0.5),
                (Type::Basic(BasicType::Bug), 0.5),
                (Type::Basic(BasicType::Ghost), 2.0),
                (Type::Basic(BasicType::Dark), 2.0),
            ]),
        ),
        (
            Type::Basic(BasicType::Dragon),
            HashMap::from([
                (Type::Basic(BasicType::Fire), 0.5),
                (Type::Basic(BasicType::Water), 0.5),
                (Type::Basic(BasicType::Electric), 0.5),
                (Type::Basic(BasicType::Grass), 0.5),
                (Type::Basic(BasicType::Ice), 2.0),
                (Type::Basic(BasicType::Dragon), 2.0),
                (Type::Basic(BasicType::Fairy), 2.0),
            ]),
        ),
        (
            Type::Basic(BasicType::Dark),
            HashMap::from([
                (Type::Basic(BasicType::Fighting), 2.0),
                (Type::Basic(BasicType::Psychic), 0.0),
                (Type::Basic(BasicType::Bug), 2.0),
                (Type::Basic(BasicType::Ghost), 0.5),
                (Type::Basic(BasicType::Dark), 0.5),
                (Type::Basic(BasicType::Fairy), 2.0),
            ]),
        ),
        (
            Type::Basic(BasicType::Steel),
            HashMap::from([
                (Type::Basic(BasicType::Normal), 0.5),
                (Type::Basic(BasicType::Fire), 2.0),
                (Type::Basic(BasicType::Grass), 0.5),
                (Type::Basic(BasicType::Ice), 0.5),
                (Type::Basic(BasicType::Fighting), 2.0),
                (Type::Basic(BasicType::Poison), 0.0),
                (Type::Basic(BasicType::Ground), 2.0),
                (Type::Basic(BasicType::Flying), 0.5),
                (Type::Basic(BasicType::Psychic), 0.5),
                (Type::Basic(BasicType::Bug), 0.5),
                (Type::Basic(BasicType::Rock), 0.5),
                (Type::Basic(BasicType::Dragon), 0.5),
                (Type::Basic(BasicType::Steel), 0.5),
                (Type::Basic(BasicType::Fairy), 0.5),
            ]),
        ),
        (
            Type::Basic(BasicType::Fairy),
            HashMap::from([
                (Type::Basic(BasicType::Fighting), 0.5),
                (Type::Basic(BasicType::Poison), 2.0),
                (Type::Basic(BasicType::Bug), 0.5),
                (Type::Basic(BasicType::Dragon), 0.0),
                (Type::Basic(BasicType::Dark), 0.5),
                (Type::Basic(BasicType::Steel), 2.0),
            ]),
        ),
    ])
}