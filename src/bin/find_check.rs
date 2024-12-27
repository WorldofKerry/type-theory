use type_theory::{analysis::checks::counters, pokemon::Pokemon, typing::{BasicType, TypeTrait}
};

/// Given a team, finds appropriate checks for an opposing Pokemon
fn main() {
    let team: Vec<Pokemon> = serde_json::from_str(r#"[{"species":"Poliwrath","typing":["Water","Fighting"],"ability":null,"moves":[]},{"species":"Numel","typing":["Fire","Ground"],"ability":null,"moves":[]},{"species":"Emolga","typing":["Electric","Flying"],"ability":"MotorDrive","moves":[]},{"species":"Nuzleaf","typing":["Grass","Dark"],"ability":null,"moves":[]},{"species":"Pawniard","typing":["Dark","Steel"],"ability":null,"moves":[]},{"species":"Frillish","typing":["Water","Ghost"],"ability":"WaterAbsorb","moves":[]}]"#).unwrap();

    use BasicType::*;
let opposing_pokemon = Pokemon::from((Poison, Dark));
    team.iter()
        .filter(|p| {
            counters(p, &opposing_pokemon)
        })
        .for_each(|p| {
            let stab_resistance: Vec<(BasicType, f32)> = opposing_pokemon
                .typing
                .iter()
                .map(|t| (*t, p.defense().get(*t)))
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
