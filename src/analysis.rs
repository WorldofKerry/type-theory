use autoscale::AutoScale;
use rand::Rng;

use crate::pokemon::Pokemon;

pub mod team_opp_matchup;
mod resistance_connector;
mod complement_matrix;
mod complement_cycle;
mod average_resistance;
pub mod autoscale;
mod offensive_coverage;

pub fn random_neighbour(team: Vec<Pokemon>, pool: &Vec<Pokemon>) -> Vec<Pokemon> {
    let mut team = team.clone();
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..team.len());
    team[index] = Pokemon::random(pool);
    team
}

pub fn score<const N: usize>(team: &Vec<Pokemon>, opponents: &Vec<Pokemon>) -> [f64; N] {
    // let matchup_score = team_opp_matchup::score_team_opp_matchup(&team, &opponents, team_opp_matchup::good_matchup);            
    // let synergy_score = complement_matrix::create_complement_matrix(&team).values().map(|m| m.values().sum::<i32>()).map(|s| s as f64).sum::<f64>();
    let resistance_score = average_resistance::resistance_count(&team);
    let resistance_multiplier = average_resistance::resistance_multiplier(&team, 0.25);
    let resistance_balance = average_resistance::resistance_balance(&team);
    let offensive_coverage = offensive_coverage::score_offensive_coverage(&team);
    let mut ret: [f64; N] = [0.0; N];
    ret[0] = resistance_balance;
    ret[1] = offensive_coverage;
    ret[2] = resistance_score;
    ret[3] = resistance_multiplier;
    // ret[4] = synergy_score;
    // ret[5] = matchup_score;
    ret
}

pub fn simulated_annealing<const N: usize>(team: Vec<Pokemon>, pool: &Vec<Pokemon>, autoscale: AutoScale::<N>) -> Vec<Pokemon> {
    let mut team_best = team.clone();
    let mut team_good = team;
    let mut autoscale = autoscale;
    let mut temp = 0.5;
    let temp_step = 0.1;
    let k_max = pool.len();
    while temp >= 0.0 {
        for k in 0..k_max {
            let team_new = random_neighbour(team_good.clone(), pool);

            let opponents = Pokemon::random_team(&Pokemon::all(), 100);            
            let scores_good = score(&team_good, &opponents);
            let scores_new = score(&team_new, &opponents);
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

            let score_best = autoscale.scale(score(&team_best, &opponents));
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