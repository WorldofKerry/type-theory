use core::f64;

use type_theory::analysis::simulated_annealing;
use type_theory::pokemon::Pokemon;
use type_theory::analysis::autoscale::AutoScale;

fn main() {
    let size = 6;
    let mut autoscale = AutoScale::new([1.0, 0.5, 0.8, 1.0, 0.0, 0.0]);
    // let pool = Pokemon::from_pkhex_dump("data/Box Data Dump.csv");
    let pool = Pokemon::all();
    let mut best_team = Pokemon::random_team(&pool, size);
    autoscale.add(type_theory::analysis::score(&best_team, &Pokemon::random_team(&pool, 100)));

    loop {
        let team = simulated_annealing(Pokemon::random_team(&pool, size), &pool, autoscale.clone(), 100);
        let opponents = Pokemon::random_team(&pool, 100);

        let scores = type_theory::analysis::score(&team, &opponents);
        autoscale.add(scores);

        let score = autoscale.scale(scores);
        let best_score = autoscale.scale(type_theory::analysis::score(&best_team, &opponents));

        if score > best_score {
            best_team = team;
            print!("Global best: {scores:4.2?} ");
            best_team.iter().for_each(|p| print!("{:?} ", p.species));
            println!();
        } else {
            print!("Rejected: {scores:4.2?} ");
            team.iter().for_each(|p| print!("{:?} ", p.species));
            println!();
        }
    }
}
