use type_theory::{
    analysis::{
        checks::{self},
        offensive_coverage::{offensive_coverage, offensive_coverage_impl},
        score,
    },
    injest::parse_names,
    pokemon::Pokemon,
};

/// Given a team, finds appropriate checks for an opposing Pokemon
fn main() {
    let team: Vec<Pokemon> = serde_json::from_str(r#"[{"species":"Comfey","typing":["Fairy"],"ability":null,"moves":[]},{"species":"Excadrill","typing":["Ground","Steel"],"ability":null,"moves":[]},{"species":"Houndoom","typing":["Fire","Dark"],"ability":null,"moves":[]},{"species":"Pansage","typing":["Grass"],"ability":null,"moves":[]},{"species":"Slowpoke","typing":["Water","Psychic"],"ability":null,"moves":[]},{"species":"Zubat","typing":["Poison","Flying"],"ability":null,"moves":[]}]"#).unwrap();
    let team: Vec<Pokemon> = parse_names(vec![
        "Comfey",
        "Excadrill",
        "Houndoom",
        "Pansage",
        "Slowpoke",
        "Zubat",
    ])
    .collect();

    let score = score::<3>(&team);
    println!("Score: {:?}", score);

    let unchecked_checks = checks::counter_balance(&team);
    println!("Unchecked checks: {:?}", unchecked_checks);

    let missing_coverage = offensive_coverage_impl(&team)
        .into_iter()
        .filter(|(p, s)| *s == 0)
        .map(|(p, _)| p)
        .collect::<Vec<_>>();
    println!("Missing coverage: {:?}", missing_coverage);
}
