use std::fs::OpenOptions;

use type_theory::{
    pokemon::Pokemon,
    typing::{BasicType, TypeTrait},
};

/// Given a team, finds appropriate checks for an opposing Pokemon
fn main() {
    let team: Vec<Pokemon> = serde_json::from_str(r#"[{"species":"Jellicent","typing":["Water","Ghost"],"ability":"WaterAbsorb","moves":[{"name":"Shadow Ball","typing":"Ghost","power":80},{"name":"Will O Wisp","typing":"Fire","power":null},{"name":"Scald","typing":"Water","power":80},{"name":"Recover","typing":"Normal","power":null}]},{"species":"Toxicroak","typing":["Fighting","Poison"],"ability":"DrySkin","moves":[{"name":"Mud Bomb","typing":"Ground","power":65},{"name":"Sucker Punch","typing":"Dark","power":70},{"name":"Venoshock","typing":"Poison","power":65},{"name":"Nasty Plot","typing":"Dark","power":null}]},{"species":"Graveler","typing":["Ground","Rock"],"ability":null,"moves":[{"name":"Explosion","typing":"Normal","power":250},{"name":"Rock Slide","typing":"Rock","power":75},{"name":"Dig","typing":"Ground","power":80},{"name":"Brick Break","typing":"Fighting","power":75}]},{"species":"Cacturne","typing":["Grass","Dark"],"ability":null,"moves":[{"name":"Leech Seed","typing":"Grass","power":null},{"name":"Brick Break","typing":"Fighting","power":75},{"name":"Energy Ball","typing":"Grass","power":90},{"name":"Dark Pulse","typing":"Dark","power":80}]},{"species":"Rotom","typing":["Fire","Electric"],"ability":"Levitate","moves":[{"name":"Hex","typing":"Ghost","power":65},{"name":"Volt Switch","typing":"Electric","power":70},{"name":"Overheat","typing":"Fire","power":130},{"name":"Shock Wave","typing":"Electric","power":60}]}]"#).unwrap();

    use BasicType::*;
    let opposing_pokemon = Pokemon::from((Normal));

    // Find a pokemon that has a supereffective move against the opposing pokemon
    team.iter()
        .filter(|p| {
            // Ensure takes at least neutral damage from opposing pokemon stab moves
            opposing_pokemon.typing.iter().all(|t| p.defense().get(*t) <= 1.0)
        })
        .for_each(|p| {
            let stab_resistance: Vec<(BasicType, f32)> = opposing_pokemon
                .typing
                .iter()
                .map(|t| (t.clone(), p.defense().get(*t)))
                .collect();
            println!("{:?} {:?}", p.species, stab_resistance);
            for move_ in p.moves.iter().filter(|m| match m.power {
                Some(_) => opposing_pokemon.defense().get(m.typing) > 1.0,
                None => false,
            }) {
                println!("  {:?}", move_);
            }
            println!();
        });
}
