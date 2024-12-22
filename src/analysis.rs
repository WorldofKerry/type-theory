use autoscale::AutoScale;
use rand::Rng;

use crate::pokemon::Pokemon;

pub mod team_opp_matchup;
pub mod resistance_connector;
pub mod complement_matrix;
pub mod complement_cycle;
pub mod resistance;
pub mod autoscale;
pub mod offensive_coverage;

pub fn random_neighbour(team: Vec<Pokemon>, pool: &Vec<Pokemon>) -> Vec<Pokemon> {
    let mut team = team.clone();
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..team.len());
    team[index] = Pokemon::random(pool);
    team
}

pub fn simulated_annealing<const N: usize>(team: Vec<Pokemon>, pool: &Vec<Pokemon>, autoscale: &mut AutoScale::<N>,
    score_fn: fn(&Vec<Pokemon>) -> [f64; N]
) -> Vec<Pokemon> {
    let mut team_best = team.clone();
    let mut team_good = team;
    let mut temp = 0.5;
    let temp_step = 0.1;
    let k_max = pool.len();
    while temp >= 0.0 {
        for k in 0..k_max {
            let team_new = random_neighbour(team_good.clone(), pool);
            let scores_good = score_fn(&team_good);
            let scores_new = score_fn(&team_new);
            autoscale.add(scores_new);
            
            let score_good = autoscale.scale(scores_good);
            let score_new = autoscale.scale(scores_new);

            let delta = score_new - score_good;
            let delta = if delta.is_nan() { 0.0 } else { delta };
            let probability = (delta / temp).exp();
            
            // println!("Temp: {temp:5.2?} Delta: {:5.2?}, Probability: {:5.2?}", delta, probability);
            if delta >= 0.0 || rand::Rng::gen_bool(&mut rand::thread_rng(), probability) {
                team_good = team_new;
                // println!("Accepted {temp:5.2?}: old {:?}, new {:?} ", score_good, score_new);
                // team_good.iter().for_each(|p| print!("{:?} ", p.species));
                // println!();
            }

            let score_best = autoscale.scale(score_fn(&team_best));
            if score_good > score_best {
                team_best = team_good.clone();
                // print!("Local best: temp {:?}, k {:?}, old {:?}, new {:?} ", temp, k, score_best, score_good);
                // print!("Local best: temp {temp:5.2?}, scores: {scores_new:5.2?} "); 
                // team_best.iter().for_each(|p| print!("{:?} ", p.species));
                // println!();
            }
        }
        temp -= temp_step;
    }
    team_best
}