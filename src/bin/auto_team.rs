/// Given a pkhex dump of available Pokemon, stochastically finds the best team based on a scoring function
use core::f64;
use itertools::Itertools;
use rayon::iter::ParallelIterator;
use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};
use type_theory::analysis::autoscale::AutoScale;
use type_theory::analysis::scoring::{dominates, is_better};
use type_theory::analysis::{checks, offensive_coverage, resistance, score, simulated_annealing};
use type_theory::injest::{parse_names_file, parse_pkhex_dump};
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
        let mut discard = false;
        for other_team in teams {
            if team == other_team {
                continue;
            }
            let other_team_scores = score(other_team);
            let mut dominated = true;
            let mut identical = true;
            for (team_score, other_team_score) in team_scores.iter().zip(other_team_scores.iter()) {
                if team_score > other_team_score {
                    dominated = false;
                    break;
                }
                if team_score != other_team_score {
                    identical = false;
                }
            }
            if dominated && !identical {
                discard = true;
                break;
            }
        }
        if !discard {
            not_dominated.insert(team.clone());
        }
    }
    not_dominated
}

fn main() {
    const SIMULATED_ANNEALING_ITERATIONS: usize = 100;
    const THREAD_COUNT: usize = 6;

    const SCORES_COUNT: usize = 4;
    let team_size = 6;
    // let pool = Pokemon::all_unique_type_chart();
    let pool = {
        // let pool = parse_pkhex_dump("Box Data Dump.csv");
        let pool = parse_names_file("unbound_pkm.txt");
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
            eprintln!("{counter:?}:");
            best_teams
                .iter()
                .map(|team| {
                    let scores = score::<SCORES_COUNT>(team);
                    (scores, team)
                })
                .sorted_by(|(scores1, _), (scores2, _)| {
                    scores1
                        .partial_cmp(scores2)
                        .unwrap_or_else(|| panic!("{:?} {:?}", scores1, scores2))
                })
                .for_each(|(scores, team)| {
                    eprint!("{scores:7.3?} ");
                    team.iter()
                        .map(|p| &p.species)
                        .sorted()
                        .for_each(|p| eprint!("{:?} ", p));
                    eprintln!();
                    println!("{}", serde_json::to_string(&team).unwrap());
                });
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use type_theory::{injest::parse_names, typing::BasicType::*};

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

    #[test]
    fn test_discard_bad_team() {
        // Ensure the inferior team is discarded
        // Depends on how the score function is defined
        let teams: BTreeSet<Vec<Pokemon>> = BTreeSet::from([
            parse_names(vec![
                "Beldum", "Comfey", "Ducklett", "Geodude", "Houndour", "Pansage",
            ])
            .collect(),
            parse_names(vec![
                "Cutiefly",
                "Drilbur",
                "Ducklett",
                "Electrike",
                "Houndour",
                "Nidoran♂ (male)",
            ])
            .collect(),
        ]);
        let after = discard_dominated_teams(score::<5>, &teams);
        for team in &after {
            eprint!("{:?}: ", score::<5>(team));
            team.iter().for_each(|p| eprint!("{:?} ", p.species));
            eprintln!();
        }
        assert_eq!(after.len(), 1);
    }

    #[test]
    fn test_discard_identital_score() {
        // Discard teams with identical
        let teams: BTreeSet<Vec<Pokemon>> = BTreeSet::from([
            parse_names(vec![
                "Beldum", "Comfey", "Ducklett", "Geodude", "Houndour", "Pansage",
            ])
            .collect(),
            parse_names(vec![
                "Beldum",
                "Comfey",
                "Ducklett",
                "Geodude",
                "Houndour",
                "Rillaboom",
            ])
            .collect(),
        ]);
        let after = discard_dominated_teams(score::<5>, &teams);
        assert_eq!(after.len(), 2); // Keep both teams if identical scores but different members
    }

    #[test]
    fn test_discard_identital_team() {
        // Discard teams with identical
        let teams: BTreeSet<Vec<Pokemon>> = BTreeSet::from([
            parse_names(vec![
                "Beldum", "Comfey", "Ducklett", "Geodude", "Houndour", "Pansage",
            ])
            .collect(),
            parse_names(vec![
                "Beldum", "Comfey", "Ducklett", "Geodude", "Houndour", "Pansage",
            ])
            .collect(),
        ]);
        let after = discard_dominated_teams(score::<5>, &teams);
        assert_eq!(after.len(), 1); // Keep both teams if identical scores but different members
    }
}
