/// Given a pkhex dump of available Pokemon, stochastically finds the best team based on a scoring function
use core::f64;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::sync::Mutex;
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
    let size = 5;
    let autoscale = Mutex::new(AutoScale::new([1.0, 0.25, 0.5, 0.75]));
    let pool = Pokemon::all();
    let pool = {
        let pool = Pokemon::from_pkhex_dump("data/Box Data Dump.csv");
        pool.iter()
            .for_each(|p| println!("{:?} {:?} {:?}", p.species, p.typing, p.ability));
        pool
    };

    let mut best_team = Pokemon::random_team(&pool, size);
    autoscale.lock().unwrap().add(score(&best_team));

    loop {
        let results: Vec<(AutoScale<4>, Vec<Pokemon>)> = (0..16)
            .into_par_iter()
            .map(|_| {
                let mut autoscale = autoscale.lock().unwrap().clone();
                let team = simulated_annealing(
                    Pokemon::random_team(&pool, size),
                    &pool,
                    &mut autoscale,
                    score,
                );
                (autoscale, team)
            })
            .collect();

        let mut autoscale = autoscale.lock().unwrap();

        for (autoscale_, team) in results {
            autoscale.combine(&autoscale_);

            let team_scores = score(&team);
            let team_score = autoscale.scale(team_scores);
            let best_score = autoscale.scale(score(&best_team));

            if team_score > best_score {
                print!("Global best: {team_scores:4.2?} ");
                team.iter()
                    .map(|p| &p.species)
                    .sorted()
                    .for_each(|p| print!("{:?} ", p));
                println!("{}", serde_json::to_string(&best_team).unwrap());
                println!();
                best_team = team.into();
            } else {
                // print!("Rejected: {scores:4.2?} ");
                // team.iter().for_each(|p| print!("{:?} ", p.species));
                // println!();
            }
        }
    }
}
