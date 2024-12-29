use type_theory::{
    analysis::checks::counters,
    pokemon::Pokemon,
    typing::{BasicType, TypeTrait},
};

/// Given a team, finds appropriate checks for an opposing Pokemon
fn main() {
    let team: Vec<Pokemon> = serde_json::from_str(r#"[{"species":"Comfey","typing":["Fairy"],"ability":null,"moves":[]},{"species":"Ducklett","typing":["Water","Flying"],"ability":null,"moves":[]},{"species":"Inkay","typing":["Psychic","Dark"],"ability":null,"moves":[]},{"species":"Electrike","typing":["Electric"],"ability":null,"moves":[]},{"species":"Wingull","typing":["Water","Flying"],"ability":null,"moves":[]},{"species":"Beldum","typing":["Psychic","Steel"],"ability":null,"moves":[]}]"#).unwrap();

    use BasicType::*;
    let opposing_pokemon = Pokemon::from((Ground));
    team.iter()
        .filter(|p| counters(p, &opposing_pokemon))
        .for_each(|p| {
            let stab_resistance: Vec<(BasicType, f32)> = opposing_pokemon
                .typing
                .iter()
                .map(|t| (*t, p.defense().get(*t)))
                .collect();
            println!("{:?} {:?}", p.species, stab_resistance);
            if p.moves.is_empty() {
                for t in p.typing.iter() {
                    if opposing_pokemon.defense().get(*t) > 1.0 {
                        println!("  {:?} STAB", t);
                    }
                }
            } else {                
                for move_ in p.moves.iter().filter(|m| match m.power {
                    Some(_) => opposing_pokemon.defense().get(m.typing) > 1.0,
                    None => false,
                }) {
                    println!("  {:?}", move_);
                }
            }
            println!();
        });
}
