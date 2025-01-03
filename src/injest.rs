use crate::{
    pokemon::{Move, Pokemon, Typing},
    typing::{combine_defense_charts, Ability, BasicType, Relationship, TypeTrait},
};
use itertools::Itertools;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, str::FromStr, sync::OnceLock};
use strum::IntoEnumIterator;

pub fn parse_pkhex_dump(file: &str) -> Vec<Pokemon> {
    // "Position","Nickname","Species","Nature","Gender","ESV","HP_Type","Ability","Move1","Move2","Move3","Move4","HeldItem","HP","ATK","DEF","SPA","SPD","SPE","MetLoc","EggLoc","Ball","OT","Version","OTLang","Legal","EC","PID","IV_HP","IV_ATK","IV_DEF","IV_SPA","IV_SPD","IV_SPE","EXP","Level","EV_HP","EV_ATK","EV_DEF","EV_SPA","EV_SPD","EV_SPE","Cool","Beauty","Cute","Smart","Tough","Sheen","NotOT","AbilityNum","GenderFlag","Form","PokerusStrain","PokerusDays","MetLevel","OriginalTrainerGender","FatefulEncounter","IsEgg","IsNicknamed","IsShiny","TID16","SID16","TSV","Move1_PP","Move2_PP","Move3_PP","Move4_PP","Move1_PPUp","Move2_PPUp","Move3_PPUp","Move4_PPUp","Relearn1","Relearn2","Relearn3","Relearn4","Checksum","Friendship","EggYear","EggMonth","EggDay","MetYear","MetMonth","MetDay"
    let all_pokemon = Pokemon::all();
    let all_moves = Move::all();
    let mut rdr = csv::Reader::from_path(file).unwrap();
    rdr.records()
        .map(|r| {
            let record = r.unwrap();
            let species = record.get(2).unwrap().to_string();
            let ability = match record.get(7).unwrap() {
                "" => None,
                a => match Ability::from_str(a) {
                    Ok(a) => Some(a),
                    Err(_) => None,
                },
            };
            let typing = {
                let matched_pokemon = all_pokemon.iter().find(|p| p.species == species).unwrap();
                let form = record.get(51).unwrap();
                if form != "0" {
                    match matched_pokemon.species.as_str() {
                        "Rotom" => match form {
                            "0" => matched_pokemon.typing.clone(),
                            "1" => Typing::from((BasicType::Electric, BasicType::Fire)),
                            "2" => Typing::from((BasicType::Electric, BasicType::Water)),
                            "3" => Typing::from((BasicType::Electric, BasicType::Ice)),
                            "4" => Typing::from((BasicType::Electric, BasicType::Flying)),
                            "5" => Typing::from((BasicType::Electric, BasicType::Grass)),
                            _ => panic!("Invalid Rotom form"),
                        },
                        "Gastrodon" => matched_pokemon.typing.clone(),
                        "Basculin" => match form {
                            "0" => Typing::from(BasicType::Water),
                            "1" => Typing::from(BasicType::Water),
                            _ => panic!("Invalid Basculin form"),
                        },
                        _ => panic!("Unhandled form for {species:?} with form {form:?}"),
                    }
                } else {
                    matched_pokemon.typing.clone()
                }
            };
            let moves = vec![
                record.get(8).unwrap(),
                record.get(9).unwrap(),
                record.get(10).unwrap(),
                record.get(11).unwrap(),
            ]
            .into_iter()
            .flat_map(|m| {
                let name = m.to_string();
                let name = name.replace("-", " ");
                if name == "(None)" {
                    vec![]
                } else {
                    vec![all_moves
                        .iter()
                        .find(|m| m.name.to_lowercase() == name.to_lowercase())
                        .unwrap_or_else(|| panic!("{name:?}"))
                        .clone()]
                }
            })
            .collect();
            Pokemon {
                species,
                typing,
                ability,
                moves,
            }
        })
        .collect()
}

/// Parses a file of Pokemon names in each line
pub fn parse_names_file(file: &str) -> Vec<Pokemon> {
    parse_names(std::fs::read_to_string(file).unwrap().lines()).collect()
}

/// Parses a Pokemon name strings to a a list of Pokemon
pub fn parse_names<'a, T: IntoIterator<Item = &'a str>>(names: T) -> impl Iterator<Item = Pokemon> + use <'a, T> {
    let all_pokemon = Pokemon::all();
    names.into_iter().flat_map(move |name| {
        let species = name.to_string();
        let matched_pokemon = all_pokemon.iter().filter(|p| p.species == species).collect::<Vec<_>>();
        if matched_pokemon.len() == 0 {
            panic!("Could not find {species:?} in the list of all Pokemon");
        }
        matched_pokemon
    }).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_from_pkhex_dump() {
        let file = "Box Data Dump.csv";
        let team = parse_pkhex_dump(file);
        assert!(team.len() >= 6);
    }
}
