use autoscale::AutoScale;
use rand::Rng;
use crate::pokemon::Pokemon;
pub mod resistance_connector;
pub mod complement_matrix;
pub mod complement_cycle;
pub mod resistance;
pub mod autoscale;
pub mod offensive_coverage;
pub mod checks;
pub mod scoring;

pub fn random_neighbour(team: Vec<Pokemon>, pool: &Vec<Pokemon>) -> Vec<Pokemon> {
    let mut team = team.clone();
    let mut rng = rand::thread_rng();
    
    let mut replacement = Pokemon::random(pool);
    while team.contains(&replacement) {
        replacement = Pokemon::random(pool);
    }
    let index = rng.gen_range(0..team.len());
    team[index] = replacement;
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
            
            if delta >= 0.0 || rand::Rng::gen_bool(&mut rand::thread_rng(), probability) {
                team_good = team_new;
            }

            let score_best = autoscale.scale(score_fn(&team_best));
            if score_good > score_best {
                team_best = team_good.clone();
            }
        }
        temp -= temp_step;
    }
    team_best
}