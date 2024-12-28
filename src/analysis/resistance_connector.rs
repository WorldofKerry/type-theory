use crate::pokemon::Pokemon;

use super::complement_matrix::resistance_complements;


pub fn resistance_connector(
    poke1: &Pokemon,
    poke2: &Pokemon,
    pool: &[Pokemon],
) -> Vec<(Pokemon, i32, i32)> {
    pool.iter()
        .map(|poke3| {
            let score1 = resistance_complements(poke3, poke1);
            let score2 = resistance_complements(poke2, poke3);
            (poke3.clone(), score1, score2)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    
    use crate::typing::BasicType::*;
    use super::*;

    #[test]
    fn test_resistance_connector() {
        let pool = Pokemon::all_type_combinations().collect::<Vec<_>>();
        let poke1 = Pokemon::from(Normal);
        let poke2 = Pokemon::from(Fire);
        let res = resistance_connector(&poke1, &poke2, &pool);
        assert!(res.len() >= 153);
    }
}