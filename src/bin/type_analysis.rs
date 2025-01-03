use std::collections::{BTreeMap, BTreeSet};
use itertools::Itertools;
use strum::IntoEnumIterator;
use type_theory::{injest::parse_pkhex_dump, pokemon::Pokemon, typing::{BasicType, TypeTrait}};

fn main() {
    // // Find all pokemon with zero or one weakness
    // let all_pokemon = Pokemon::all_unique_type_chart();
    // let mut weaknesses: BTreeMap<usize, BTreeSet<Pokemon>> = BTreeMap::new();
    // for poke in all_pokemon.iter() {
    //     let def = poke.defense();
    //     let weak_count = def.iter().filter(|(_, v)| **v > 1.0).count();
    //     weaknesses.entry(weak_count).or_default().insert(poke.clone());
    // }
    // for (weak_count, pokemon) in weaknesses.iter().rev() {
    //     println!("Weakness count: {}", weak_count);
    //     for poke in pokemon.iter() {
    //         println!("{:?}", poke);
    //     }
    // }

    // // Find all pokemon weak to both water and electric
    // let water_electric_weak: BTreeSet<Pokemon> = all_pokemon
    //     .iter()
    //     .filter(|poke| poke.defense().get(type_theory::typing::BasicType::Water) > 1.0 && poke.defense().get(type_theory::typing::BasicType::Electric) > 1.0)
    //     .cloned()
    //     .collect();
    // println!("Weak to water and electric: {:?}", water_electric_weak);

    // Find all pokemon hit not hit super effectively by 4 types
    const NUM_TYPES: usize = 6;
    let mut mappings: BTreeMap<[BasicType; NUM_TYPES], BTreeSet<Pokemon>> = BTreeMap::new();
    for types in BasicType::iter().combinations(NUM_TYPES) {
        if types.contains(&BasicType::Normal) {
            continue;
        }
        let types: Vec<BasicType> = types;
        let not_super_effective: BTreeSet<Pokemon> = Pokemon::all_unique_type_chart()
            .iter()
            .filter(|poke| {
                types.iter().all(|t| poke.defense().get(*t) <= 1.0)
            })
            .cloned()
            .collect();
        mappings.insert(types.try_into().unwrap(), not_super_effective);
    }
    let counts = mappings.iter().map(|(types, pokemons)| (pokemons.len(), types)).sorted().rev().collect::<Vec<_>>();
    for (count, types) in counts {
        println!("Count: {} {:?}", count, types);
    }
}
