use core::f64;

use type_theory::analysis::{resistance, offensive_coverage, simulated_annealing};
use type_theory::pokemon::Pokemon;
use type_theory::analysis::autoscale::AutoScale;

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
    let mut autoscale = AutoScale::new([1.0, 0.25, 0.5, 0.75]);
    let pool = Pokemon::all();
    let pool = {
        let pool = Pokemon::from_pkhex_dump("data/Box Data Dump.csv");
        pool.iter().for_each(|p| println!("{:?} {:?} {:?}", p.species, p.typing, p.ability));
        pool
    };

    let mut best_team = Pokemon::random_team(&pool, size);
    autoscale.add(score(&best_team));

    loop {
        let team = simulated_annealing(Pokemon::random_team(&pool, size), &pool, &mut autoscale, score);

        let team_scores = score(&team);
        autoscale.add(team_scores);

        let team_score = autoscale.scale(team_scores);
        let best_score = autoscale.scale(score(&best_team));

        if team_score > best_score {
            best_team = team;
            print!("Global best: {team_scores:4.2?} ");
            best_team.iter().for_each(|p| print!("{:?} ", p.species));
            println!();
        } else {
            // print!("Rejected: {scores:4.2?} ");
            // team.iter().for_each(|p| print!("{:?} ", p.species));
            // println!();
        }
    }
}
