/// Given a pkhex dump of available Pokemon, stochastically finds the best team based on a scoring function
use core::f64;
use itertools::Itertools;
use rayon::iter::ParallelIterator;
use type_theory::analysis::scoring::{is_better, dominates};
use type_theory::injest::{parse_names, parse_pkhex_dump};
use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};
use type_theory::analysis::autoscale::AutoScale;
use type_theory::analysis::{checks, offensive_coverage, resistance, simulated_annealing};
use type_theory::pokemon::Pokemon;

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
    // If another team dominates a team in all scores, discard the dominated team
    let mut not_dominated = BTreeSet::new();
    for team in teams {
        let team_scores = score(team);
        let mut dominated = false;
        for other_team in teams {
            if team == other_team {
                continue;
            }
            let other_team_scores = score(other_team);
            let mut all_dominated = true;
            for (team_score, other_team_score) in team_scores.iter().zip(other_team_scores.iter()) {
                if team_score <= other_team_score {
                    all_dominated = false;
                    break;
                }
            }
            if all_dominated {
                dominated = true;
                break;
            }
        }
        if !dominated {
            not_dominated.insert(team.clone());
        }
    }
    not_dominated
}

pub fn score<const N: usize>(team: &Vec<Pokemon>) -> [f64; N] {
    let mut ret: [f64; N] = [0.0; N];
    ret[0] = resistance::per_type_net_resist_weak_count(team);
    // ret[1] = resistance::one_resist_for_each_type(team);
    // ret[2] = resistance::per_type_multiplier(team, 0.25);
    // let random_pool = Pokemon::random_team(Pokemon::all_unique_type_chart(), 100).into_iter().collect();
    // ret[3] = checks::counter_count(team, &random_pool) as f64;
    ret[1] = offensive_coverage::offensive_coverage(team);
    ret[2] = -(checks::counter_balance(team).len() as f64);
    ret
}

fn main() {
    const SIMULATED_ANNEALING_ITERATIONS: usize = 100;
    const THREAD_COUNT: usize = 6;

    const SCORES_COUNT: usize = 3;
    let team_size = 6;
    // let pool = Pokemon::all_unique_type_chart();
    let pool = {
        // let pool = parse_pkhex_dump("Box Data Dump.csv");
        let pool = parse_names("unbound_pkm.txt");
        pool.iter()
            .for_each(|p| eprintln!("{:?} {:?} {:?}", p.species, p.typing, p.ability));
        pool
    };
    eprintln!("Pool size: {}", pool.len());

    let best_teams = Arc::new(Mutex::new(BTreeSet::new()));

    rayon::ThreadPoolBuilder::new()
        .num_threads(THREAD_COUNT)
        .build_global()
        .unwrap();

    let counter = Arc::new(Mutex::new(1));
    rayon::iter::repeatn((), SIMULATED_ANNEALING_ITERATIONS).for_each(|_| {
        let team = simulated_annealing(
            Pokemon::random_team(&pool, team_size),
            &pool,
            score::<SCORES_COUNT>,
        );
        best_teams.lock().unwrap().insert(team);
        let mut counter = counter.lock().unwrap();
        *counter += 1;

        if *counter % THREAD_COUNT == 0 || *counter == SIMULATED_ANNEALING_ITERATIONS {
            let mut best_teams = best_teams.lock().unwrap();
            *best_teams = discard_dominated_teams(score::<SCORES_COUNT>, &best_teams);
            println!("{counter:?}:");
            best_teams
                .iter()
                .map(|team| {
                    let scores = score::<SCORES_COUNT>(team);
                    (scores, team)
                })
                .sorted_by(|(scores1, _), (scores2, _)| {
                    scores1.partial_cmp(scores2).unwrap_or_else(|| panic!("{:?} {:?}", scores1, scores2))
                })
                .for_each(|(scores, team)| {
                    eprint!("{scores:7.3?} ");
                    team.iter()
                        .map(|p| &p.species)
                        .sorted()
                        .for_each(|p| eprint!("{:?} ", p));
                    eprintln!();
                });
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use type_theory::typing::BasicType::*;

    #[test]
    fn test_score_trio_cores() {
        let team = vec![
            Pokemon::from((Fire)),
            Pokemon::from((Water)),
            Pokemon::from((Grass)),
        ];
        let scores = score::<5>(&team);
        println!("{scores:?}");
    }

    #[test]
    fn test_basic_team() {
        let team = vec![
            Pokemon::from((Water, Flying)),
            Pokemon::from((Grass, Steel)),
            Pokemon::from((Fire)),
            Pokemon::from((Water)),
            Pokemon::from((Dragon, Water)),
            Pokemon::from((Dragon, Ghost)),
        ];
        let scores = score::<5>(&team);
        println!("{scores:?}");
    }
}
