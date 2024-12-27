use type_theory::{analysis::checks::{self, counters}, pokemon::Pokemon, typing::{BasicType, TypeTrait}
};
use type_theory::analysis::checks::checks;

/// Given a team, finds appropriate checks for an opposing Pokemon
fn main() {
    let team: Vec<Pokemon> = serde_json::from_str(r#"[{"species":"Tyranitar","typing":["Rock","Dark"],"ability":null,"moves":[]},{"species":"Croagunk","typing":["Fighting","Poison"],"ability":"DrySkin","moves":[]},{"species":"Gyarados","typing":["Water","Flying"],"ability":null,"moves":[]},{"species":"Bronzor","typing":["Psychic","Steel"],"ability":"Levitate","moves":[]},{"species":"Cacturne","typing":["Grass","Dark"],"ability":"WaterAbsorb","moves":[]},{"species":"Rotom","typing":["Fire","Electric"],"ability":"Levitate","moves":[]}]"#).unwrap();

    use BasicType::*;
    let unchecked_checks = checks::counter_balance(&team);
    println!("Unchecked checks: {:?}", unchecked_checks);
}
