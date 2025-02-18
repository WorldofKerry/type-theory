use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use std::sync::OnceLock;
use strum::EnumIter;
use strum::EnumString;
use strum::IntoEnumIterator;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    EnumIter,
    Ord,
    PartialOrd,
    EnumString,
    Serialize,
    Deserialize,
)]
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
    #[cfg(feature = "gen6")]
    Fairy,
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    EnumIter,
    Ord,
    PartialOrd,
    EnumString,
    Serialize,
    Deserialize,
)]
pub enum Ability {
    #[strum(serialize = "Levitate")]
    Levitate,
    #[strum(serialize = "Heatproof")]
    Heatproof,
    #[strum(serialize = "Water Absorb")]
    WaterAbsorb,
    #[strum(serialize = "Dry Skin")]
    DrySkin,
    #[strum(serialize = "Flash Fire")]
    FlashFire,
    #[strum(serialize = "Thick Fat")]
    ThickFat,
    #[strum(serialize = "Volt Absorb")]
    VoltAbsorb,
    #[strum(serialize = "Lightning Rod")]
    LightningRod,
    #[strum(serialize = "Sap Sipper")]
    SapSipper,
    #[strum(serialize = "Motor Drive")]
    MotorDrive,
    #[strum(serialize = "Storm Drain")]
    StormDrain,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Type {
    Basic(BasicType),
    Ability(Ability),
}

impl IntoEnumIterator for Type {
    type Iterator = std::vec::IntoIter<Self>;

    fn iter() -> Self::Iterator {
        BasicType::iter()
            .map(Type::Basic)
            .chain(Ability::iter().map(Type::Ability))
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl TypeTrait for BasicType {
    fn defense(&self) -> Relationship {
        Relationship::from_raw_parts(
            get_defense_chart()
                .get(&Type::Basic(*self))
                .unwrap()
                .clone(),
        )
    }
}

impl TypeTrait for Ability {
    fn defense(&self) -> Relationship {
        Relationship::from_raw_parts(
            get_defense_chart()
                .get(&Type::Ability(*self))
                .unwrap_or_else(|| panic!("{:?} not found", self))
                .clone(),
        )
    }
}

pub trait TypeTrait {
    fn defense(&self) -> Relationship;
}

impl TypeTrait for Type {
    fn defense(&self) -> Relationship {
        match self {
            Type::Basic(t) => t.defense(),
            Type::Ability(a) => a.defense(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Relationship {
    inner: BTreeMap<BasicType, f32>,
}

impl Relationship {
    fn from_raw_parts(inner: BTreeMap<BasicType, f32>) -> Self {
        let mut ret = Relationship { inner };
        ret.inner.retain(|_, v| *v != 1.0);
        ret
    }
    pub fn get(&self, key: BasicType) -> f32 {
        *self.inner.get(&key).unwrap_or(&1.0)
    }
    pub fn iter(&self) -> impl Iterator<Item = (&BasicType, &f32)> {
        self.inner.iter()
    }
}

pub fn combine_defense_charts(charts: impl IntoIterator<Item = Relationship>) -> Relationship {
    let mut combined_chart = BTreeMap::new();
    for chart in charts {
        for (basic_type, multiplier) in chart.inner {
            let entry = combined_chart.entry(basic_type).or_insert(1.0);
            *entry *= multiplier;
        }
    }
    Relationship::from_raw_parts(combined_chart)
}

pub fn combine_defense_charts_immune(
    charts: impl IntoIterator<Item = Relationship>,
    immune_multiplier: f32,
) -> Relationship {
    let mut combined_chart = BTreeMap::new();
    for chart in charts {
        for t in BasicType::iter() {
            let entry = combined_chart.entry(t).or_insert(1.0);
            *entry *= if chart.get(t) == 0.0 {
                immune_multiplier
            } else {
                chart.get(t)
            };
        }
    }
    Relationship::from_raw_parts(combined_chart)
}

pub fn get_multitype_defense_chart<'a>(types: impl Iterator<Item = &'a Type>) -> Relationship {
    combine_defense_charts(types.map(|t| t.defense()))
}

fn get_defense_chart() -> &'static BTreeMap<Type, BTreeMap<BasicType, f32>> {
    static DEFENSE_CHART: OnceLock<BTreeMap<Type, BTreeMap<BasicType, f32>>> = OnceLock::new();
    DEFENSE_CHART.get_or_init(|| {
        BTreeMap::from([
            (
                Type::Basic(BasicType::Normal),
                BTreeMap::from([((BasicType::Fighting), 2.0), ((BasicType::Ghost), 0.0)]),
            ),
            (
                Type::Basic(BasicType::Fire),
                BTreeMap::from([
                    ((BasicType::Fire), 0.5),
                    ((BasicType::Water), 2.0),
                    ((BasicType::Grass), 0.5),
                    ((BasicType::Ice), 0.5),
                    ((BasicType::Ground), 2.0),
                    ((BasicType::Bug), 0.5),
                    ((BasicType::Rock), 2.0),
                    ((BasicType::Steel), 0.5),
                    #[cfg(feature = "gen6")]
                    ((BasicType::Fairy), 0.5),
                ]),
            ),
            (
                Type::Basic(BasicType::Water),
                BTreeMap::from([
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
                BTreeMap::from([
                    ((BasicType::Electric), 0.5),
                    ((BasicType::Ground), 2.0),
                    ((BasicType::Flying), 0.5),
                    ((BasicType::Steel), 0.5),
                ]),
            ),
            (
                Type::Basic(BasicType::Grass),
                BTreeMap::from([
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
                BTreeMap::from([
                    ((BasicType::Fire), 2.0),
                    ((BasicType::Ice), 0.5),
                    ((BasicType::Fighting), 2.0),
                    ((BasicType::Rock), 2.0),
                    ((BasicType::Steel), 2.0),
                ]),
            ),
            (
                Type::Basic(BasicType::Fighting),
                BTreeMap::from([
                    ((BasicType::Flying), 2.0),
                    ((BasicType::Psychic), 2.0),
                    ((BasicType::Bug), 0.5),
                    ((BasicType::Rock), 0.5),
                    ((BasicType::Dark), 0.5),
                    #[cfg(feature = "gen6")]
                    ((BasicType::Fairy), 2.0),
                ]),
            ),
            (
                Type::Basic(BasicType::Poison),
                BTreeMap::from([
                    ((BasicType::Grass), 0.5),
                    ((BasicType::Fighting), 0.5),
                    ((BasicType::Poison), 0.5),
                    ((BasicType::Ground), 2.0),
                    ((BasicType::Psychic), 2.0),
                    ((BasicType::Bug), 0.5),
                    #[cfg(feature = "gen6")]
                    ((BasicType::Fairy), 0.5),
                ]),
            ),
            (
                Type::Basic(BasicType::Ground),
                BTreeMap::from([
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
                BTreeMap::from([
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
                BTreeMap::from([
                    ((BasicType::Fighting), 0.5),
                    ((BasicType::Psychic), 0.5),
                    ((BasicType::Bug), 2.0),
                    ((BasicType::Ghost), 2.0),
                    ((BasicType::Dark), 2.0),
                ]),
            ),
            (
                Type::Basic(BasicType::Bug),
                BTreeMap::from([
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
                BTreeMap::from([
                    ((BasicType::Normal), 0.5),
                    ((BasicType::Fire), 0.5),
                    ((BasicType::Water), 2.0),
                    ((BasicType::Grass), 2.0),
                    ((BasicType::Fighting), 2.0),
                    ((BasicType::Poison), 0.5),
                    ((BasicType::Ground), 2.0),
                    ((BasicType::Flying), 0.5),
                    ((BasicType::Steel), 2.0),
                ]),
            ),
            (
                Type::Basic(BasicType::Ghost),
                BTreeMap::from([
                    ((BasicType::Normal), 0.0),
                    ((BasicType::Fighting), 0.0),
                    ((BasicType::Poison), 0.5),
                    ((BasicType::Bug), 0.5),
                    ((BasicType::Ghost), 2.0),
                    ((BasicType::Dark), 2.0),
                ]),
            ),
            (
                Type::Basic(BasicType::Dragon),
                BTreeMap::from([
                    ((BasicType::Fire), 0.5),
                    ((BasicType::Water), 0.5),
                    ((BasicType::Electric), 0.5),
                    ((BasicType::Grass), 0.5),
                    ((BasicType::Ice), 2.0),
                    ((BasicType::Dragon), 2.0),
                    #[cfg(feature = "gen6")]
                    ((BasicType::Fairy), 2.0),
                ]),
            ),
            (
                Type::Basic(BasicType::Dark),
                BTreeMap::from([
                    ((BasicType::Fighting), 2.0),
                    ((BasicType::Psychic), 0.0),
                    ((BasicType::Bug), 2.0),
                    ((BasicType::Ghost), 0.5),
                    ((BasicType::Dark), 0.5),
                    #[cfg(feature = "gen6")]
                    ((BasicType::Fairy), 2.0),
                ]),
            ),
            (
                Type::Basic(BasicType::Steel),
                BTreeMap::from([
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
                    #[cfg(feature = "gen6")]
                    ((BasicType::Fairy), 0.5),
                ]),
            ),
            #[cfg(feature = "gen6")]
            (
                Type::Basic(BasicType::Fairy),
                BTreeMap::from([
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
                BTreeMap::from([((BasicType::Ground), 0.0)]),
            ),
            (
                Type::Ability(Ability::WaterAbsorb),
                BTreeMap::from([((BasicType::Water), 0.0)]),
            ),
            (
                Type::Ability(Ability::DrySkin),
                BTreeMap::from([((BasicType::Water), 0.0), ((BasicType::Fire), 1.25)]),
            ),
            (
                Type::Ability(Ability::FlashFire),
                BTreeMap::from([((BasicType::Fire), 0.0)]),
            ),
            (
                Type::Ability(Ability::ThickFat),
                BTreeMap::from([((BasicType::Fire), 0.5), ((BasicType::Ice), 0.5)]),
            ),
            (
                Type::Ability(Ability::VoltAbsorb),
                BTreeMap::from([((BasicType::Electric), 0.0)]),
            ),
            (
                Type::Ability(Ability::LightningRod),
                BTreeMap::from([((BasicType::Electric), 0.0)]),
            ),
            (
                Type::Ability(Ability::SapSipper),
                BTreeMap::from([((BasicType::Grass), 0.0)]),
            ),
            (
                Type::Ability(Ability::MotorDrive),
                BTreeMap::from([((BasicType::Electric), 0.0)]),
            ),
            (
                Type::Ability(Ability::StormDrain),
                BTreeMap::from([((BasicType::Water), 0.0)]),
            ),
            (
                Type::Ability(Ability::Heatproof),
                BTreeMap::from([((BasicType::Fire), 0.5)]),
            ),
        ])
    })
}
