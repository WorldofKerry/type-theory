use type_theory::{analysis::checks::{self}, injest::parse_names, pokemon::Pokemon
};

/// Given a team, finds appropriate checks for an opposing Pokemon
fn main() {
    let team: Vec<Pokemon> = serde_json::from_str(r#"[{"species":"Comfey","typing":["Fairy"],"ability":null,"moves":[]},{"species":"Ducklett","typing":["Water","Flying"],"ability":null,"moves":[]},{"species":"Inkay","typing":["Psychic","Dark"],"ability":null,"moves":[]},{"species":"Electrike","typing":["Electric"],"ability":null,"moves":[]},{"species":"Wingull","typing":["Water","Flying"],"ability":null,"moves":[]},{"species":"Beldum","typing":["Psychic","Steel"],"ability":null,"moves":[]}]"#).unwrap();

    
    let unchecked_checks = checks::counter_balance(&team);
    println!("Unchecked checks: {:?}", unchecked_checks);
}
