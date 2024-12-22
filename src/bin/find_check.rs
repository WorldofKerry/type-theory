use type_theory::{
    pokemon::Pokemon,
    typing::{BasicType, TypeTrait},
};

/// Given a team, finds appropriate checks for an opposing Pokemon
fn main() {
    let team: Vec<Pokemon> = serde_json::from_str(r#"[{"species":"Relicanth","typing":["Water","Rock"],"ability":null,"moves":[{"name":"Rock Slide","typing":"Rock","power":75},{"name":"Double-Edge","typing":"Normal","power":120},{"name":"Bulldoze","typing":"Ground","power":60},{"name":"Aqua Tail","typing":"Water","power":90}]},{"species":"Jellicent","typing":["Water","Ghost"],"ability":"WaterAbsorb","moves":[{"name":"Psychic","typing":"Psychic","power":90},{"name":"Will O Wisp","typing":"Fire","power":null},{"name":"Scald","typing":"Water","power":80},{"name":"Recover","typing":"Normal","power":null}]},{"species":"Rotom","typing":["Fire","Electric"],"ability":"Levitate","moves":[{"name":"Pain Split","typing":"Normal","power":null},{"name":"Volt Switch","typing":"Electric","power":70},{"name":"Overheat","typing":"Fire","power":130},{"name":"Shadow Ball","typing":"Ghost","power":80}]},{"species":"Toxicroak","typing":["Fighting","Poison"],"ability":"DrySkin","moves":[{"name":"Swords Dance","typing":"Normal","power":null},{"name":"Thunder Punch","typing":"Electric","power":75},{"name":"Drain Punch","typing":"Fighting","power":75},{"name":"Bounce","typing":"Flying","power":90}]},{"species":"Gliscor","typing":["Ground","Flying"],"ability":null,"moves":[{"name":"Swords Dance","typing":"Normal","power":null},{"name":"Ice Fang","typing":"Ice","power":65},{"name":"Roost","typing":"Flying","power":null},{"name":"Dig","typing":"Ground","power":80}]},{"species":"Hydreigon","typing":["Dragon","Dark"],"ability":"Levitate","moves":[{"name":"Fire Blast","typing":"Fire","power":110},{"name":"Dragon Pulse","typing":"Dragon","power":85},{"name":"Dark Pulse","typing":"Dark","power":80},{"name":"Surf","typing":"Water","power":90}]}]"#).unwrap();

    use BasicType::*;
let opposing_pokemon = Pokemon::from((Poison, Dark));

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
