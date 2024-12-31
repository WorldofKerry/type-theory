use crate::pokemon::Pokemon;
use autoscale::AutoScale;
use itertools::Itertools;
use rand::Rng;
use scoring::{dominates, is_better};
pub mod autoscale;
pub mod checks;
pub mod complement_cycle;
pub mod complement_matrix;
pub mod offensive_coverage;
pub mod resistance;
pub mod resistance_connector;
pub mod scoring;

pub fn score<const N: usize>(team: &Vec<Pokemon>) -> [f64; N] {
    let mut ret: [f64; N] = [0.0; N];
    ret[0] = resistance::per_type_net_resist_weak_count(team);
    // ret[1] = resistance::one_resist_for_each_type(team);
    // ret[2] = resistance::per_type_multiplier(team, 0.25);
    // let random_pool = Pokemon::random_team(Pokemon::all_unique_type_chart(), 100).into_iter().collect();
    // ret[3] = checks::counter_count(team, &random_pool) as f64;
    ret[1] = offensive_coverage::offensive_coverage(team);
    ret[2] = -(checks::counter_balance(team).len() as f64);

    // Require specific Pokemon
    ret[3] = ["Excadrill", "Wingull", "Zapdos"]
        .iter()
        .all(|species| team.iter().any(|poke| poke.species == *species)) as i32 as f64;
    ret
}

pub fn random_neighbour(team: Vec<Pokemon>, pool: &Vec<Pokemon>) -> Vec<Pokemon> {
    let mut team = team.clone();
    let mut rng = rand::thread_rng();

    let mut replacement = Pokemon::random(pool);
    while team.contains(&replacement) {
        replacement = Pokemon::random(pool);
    }
    let index = rng.gen_range(0..team.len());
    team[index] = replacement;
    team.into_iter().sorted().collect()
}

pub fn simulated_annealing<const N: usize>(
    team: Vec<Pokemon>,
    pool: &Vec<Pokemon>,
    score_fn: fn(&Vec<Pokemon>) -> [f64; N],
) -> Vec<Pokemon> {
    let mut team_best = team.clone();
    let mut team_good = team;
    let mut temp = 0.1;
    let temp_step = 0.1;
    let k_max = pool.len();
    while temp > 0.0 {
        for k in 0..k_max {
            let team_new = random_neighbour(team_good.clone(), pool);
            let scores_good = score_fn(&team_good);
            let scores_new = score_fn(&team_new);

            let delta = is_better(&scores_new, &scores_good) as f64;
            let probability = (delta / temp).exp();
            // println!(
            //     "scores_good: {:6.2?}, scores_new: {:6.2?}, delta: {:6.2}, probability: {:6.2}, temperature: {:6.2}",
            //     scores_good, scores_new, delta, probability, temp
            // );

            if delta > 0.0 || rand::Rng::gen_bool(&mut rand::thread_rng(), probability) {
                team_good = team_new;
            }

            if is_better(&scores_good, &score_fn(&team_best)) > 0 {
                team_best = team_good.clone();
            }
        }
        temp -= temp_step;
    }
    team_best
}
