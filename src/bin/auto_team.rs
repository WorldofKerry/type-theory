/// Given a pkhex dump of available Pokemon, stochastically finds the best team based on a scoring function
use core::f64;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::sync::{Arc, Mutex};
use type_theory::analysis::autoscale::AutoScale;
use type_theory::analysis::{offensive_coverage, resistance, simulated_annealing};
use type_theory::pokemon::Pokemon;

pub fn score<const N: usize>(team: &Vec<Pokemon>) -> [f64; N] {
    let mut ret: [f64; N] = [0.0; N];
    ret[0] = resistance::per_type_net_resist_weak_count(&team);
    ret[1] = offensive_coverage::offensive_coverage(&team);
    ret[2] = resistance::one_resist_for_each_type(&team);
    ret[3] = resistance::per_type_multiplier(&team, 0.25);
    ret
}

fn main() {
    let size = 6;
    let autoscale_global = Mutex::new(AutoScale::new([1.0, 0.25, 0.5, 0.75]));
    let pool = Pokemon::all_unique_type_chart().collect::<Vec<_>>();
    // let pool = {
    //     let pool = Pokemon::from_pkhex_dump("Box Data Dump.csv");
    //     pool.iter()
    //         .for_each(|p| eprintln!("{:?} {:?} {:?}", p.species, p.typing, p.ability));
    //     pool
    // };
    eprintln!("Pool size: {}", pool.len());

    let best_team = Arc::new(Mutex::new(Pokemon::random_team(&pool, size)));
    autoscale_global
        .lock()
        .unwrap()
        .add(score(&best_team.lock().unwrap()));

    rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build_global()
        .unwrap();

    let counter = Arc::new(Mutex::new(1));
    rayon::iter::repeat(()).for_each(|_| {
        let mut autoscale = autoscale_global.lock().unwrap().clone();
        let team = simulated_annealing(
            Pokemon::random_team(&pool, size),
            &pool,
            &mut autoscale,
            score,
        );
        autoscale_global.lock().unwrap().combine(&autoscale);

        let autoscale = autoscale_global.lock().unwrap();
        let team_scores = score(&team);
        let team_score = autoscale.scale(team_scores);
        let best_score = autoscale.scale(score(&best_team.lock().unwrap()));

        if team_score > best_score {
            let mut counter = counter.lock().unwrap();
            eprint!("{:?}: Global best: {team_scores:4.2?} ", counter);
            team.iter()
                .map(|p| &p.species)
                .sorted()
                .for_each(|p| eprint!("{:?} ", p));
            eprintln!();
            println!("{}", serde_json::to_string(&team).unwrap());
            eprintln!();
            best_team.lock().unwrap().clone_from(&team);
            *counter += 1;
        } else {
            // print!("Rejected: {scores:4.2?} ");
            // team.iter().for_each(|p| print!("{:?} ", p.species));
            // println!();
        }
    })
}
