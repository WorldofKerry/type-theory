mod team_opp_matchup;
mod resistance_connector;
mod complement_matrix;
mod complement_cycle;
mod average_resistance;
mod autoscale;
mod offensive_coverage;

#[cfg(test)]
mod tests {
    use team_opp_matchup::{good_matchup, score_team_opp_matchup};

    use crate::{analysis::complement_matrix::create_complement_matrix, pokemon::Pokemon, team::Team};
    use super::*;
    #[test]
    fn find_good_team() {
        let mut autoscale = autoscale::AutoScale::new([1.0, 0.5, 0.8, 1.0]);

        loop {
            let team = Pokemon::random_team(&Pokemon::all(), 6);
            let opponents = Pokemon::random_team(&Pokemon::all(), 100);
            
            let matchup_score = score_team_opp_matchup(&team, &opponents, good_matchup);            
            let synergy_score = create_complement_matrix(&team).values().map(|m| m.values().sum::<i32>()).map(|s| s as f64).sum::<f64>();
            let resistance_score = average_resistance::resistance_count(&team);
            let resistance_multiplier = average_resistance::resistance_multiplier(&team, 0.25);
            let resistance_balance = average_resistance::resistance_balance(&team);
            let offensive_coverage = offensive_coverage::score_offensive_coverage(&team);

            // println!("Matchup: {} Synergy: {} Resistance: {} IsBetter: {}", matchup_score, synergy_score, resistance_score, isbetter);
            if let Some(entry) = autoscale.add([resistance_balance, offensive_coverage, resistance_score, resistance_multiplier]) {
                let best_team = team;
                // println!("{entry:?}");
                for poke in &best_team {
                    let complement_matrix = create_complement_matrix(&best_team);
                    let best_resistance_complement_for_poke = complement_matrix.get(poke).unwrap().iter().max_by_key(|(_, &v)| v).unwrap().0;
                    println!("{:?} -> {:?}", poke, best_resistance_complement_for_poke);
                }
            }
        }
    }
}