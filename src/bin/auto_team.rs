/// Given a pkhex dump of available Pokemon, stochastically finds the best team based on a scoring function
use core::f64;
use std::collections::BTreeSet;
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

fn compute_best_team<const N: usize>(
    autoscale: &AutoScale<N>,
    score: fn(&Vec<Pokemon>) -> [f64; N],
    teams: &BTreeSet<Vec<Pokemon>>,
) -> (Vec<Pokemon>, [f64; N]) {
    let mut best_team = Vec::new();
    let mut best_score = f64::NEG_INFINITY;
    let mut best_scores = [0.0; N];
    for team in teams {
        let team_scores = score(team);
        let team_score = autoscale.scale(team_scores);
        if team_score > best_score {
            best_team = team.clone();
            best_score = team_score;
            best_scores = team_scores;
        }
    }
    (best_team, best_scores)
}

fn discard_dominated_teams<const N: usize>(
    score: fn(&Vec<Pokemon>) -> [f64; N],
    teams: &BTreeSet<Vec<Pokemon>>,
) -> BTreeSet<Vec<Pokemon>> {
    // If another team has higher score in all dimensions, discard this team
    let mut teams = teams.clone();
    for team in teams.clone() {
        teams.retain(|other| {
            let team_scores = score(&team);
            let other_scores = score(other);
            let is_dominated = (0..N).all(|i| team_scores[i] > other_scores[i]);
            !is_dominated
        });
    }
    teams
}

fn main() {
    const N: usize = 4;
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

    let best_teams = Arc::new(Mutex::new(BTreeSet::from([Pokemon::random_team(&pool, size)])));
    autoscale_global
        .lock()
        .unwrap()
        .add(score(&best_teams.lock().unwrap().iter().next().unwrap()));

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
        best_teams.lock().unwrap().insert(team);
        let mut counter = counter.lock().unwrap();
        *counter += 1;

        if *counter % 4 == 0 {
            let mut best_teams = best_teams.lock().unwrap();
            *best_teams = discard_dominated_teams(score::<N>, &*best_teams);
            let autoscale = autoscale_global.lock().unwrap();
            let (best_team, best_scores) = compute_best_team(
                &autoscale,
                score,
                &best_teams,
            );
            eprint!("{:?}: Global best: {best_scores:4.2?} ", counter);
            best_team.iter()
                .map(|p| &p.species)
                .sorted()
                .for_each(|p| eprint!("{:?} ", p));
            eprintln!();
            println!("{}", serde_json::to_string(&best_team).unwrap());
            eprintln!();
        }
    })
}
