use std::collections::HashMap;
use itertools::Itertools;
use strum::IntoEnumIterator;
use crate::{pokemon::Pokemon, typing::{BasicType, TypeTrait}};

pub fn resistance_complements(poke1: &Pokemon, poke2: &Pokemon) -> i32 {
    // How well poke1 resists weaknesses of poke2
    // Higher is better
    const IMMUNITY_LOG: i32 = -2; // treat immunity as 1/8 resistance
    let poke1_def = poke1.defense();
    let poke2_def = poke2.defense();
    let mut score = 0;
    for t in BasicType::iter() {
        let r1 = if poke1_def.get(t) == 0.0 {
            IMMUNITY_LOG
        } else {
            poke1_def.get(t).log2() as i32
        };
        let r2 = if poke2_def.get(t) == 0.0 {
            IMMUNITY_LOG
        } else {
            poke2_def.get(t).log2() as i32
        };
        if r2 > 0 {
            // println!("{t:?} {r1:?} {r2:?}");
            if r1 == 0 {
                score += 1;
            } else if r1 < 0 {
                score += 2;
            }
        }
    }
    score
}

pub fn create_complement_matrix(pool: &Vec<Pokemon>) -> HashMap<Pokemon, HashMap<Pokemon, i32>> {
    let mut result: HashMap<Pokemon, HashMap<Pokemon, i32>> = HashMap::new();
    let combinations = pool.iter().combinations(2);
    for combination in combinations {
        let (p1, p2) = (combination[0], combination[1]);
        result
            .entry(p1.clone())
            .or_default()
            .insert(p2.clone(), resistance_complements(p1, p2));
        result
            .entry(p2.clone())
            .or_default()
            .insert(p1.clone(), resistance_complements(p2, p1));
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::typing::BasicType;
    use super::*;

    #[test]
    fn create_complement_matrix_test() {
        use BasicType::*;
        let ludicolo = Pokemon::from((Grass, Water));
        let primal_groundon = Pokemon::from((Ground, Fire));
        assert_eq!(resistance_complements(&ludicolo, &primal_groundon), 4);
        assert_eq!(resistance_complements(&primal_groundon, &ludicolo), 5);
        let matrix = create_complement_matrix(&vec![ludicolo, primal_groundon]);
        println!("{matrix:?}");
    }

    #[test]
    fn find_complements() {
        use BasicType::*;
        let poke = Pokemon::from(Water);
        Pokemon::all()
            .into_iter()
            .map(|p| (p.clone(), resistance_complements(&poke, &p)))
            .unique()
            .filter(|(_, score)| *score >= 2)
            .for_each(|p| {
                println!("{:?}", p);
            });
    }
    
    #[test]
    fn find_complemented() {
        use BasicType::*;
        let poke = Pokemon::from((Grass, Ice));
        // vec![Pokemon {
        //     typing: (Poison, Ghost).into(),
        //     ability: None,
        // }]
        Pokemon::all()
            .into_iter()
            .map(|p| (p.clone(), resistance_complements(&p, &poke)))
            .unique()
            .filter(|(_, score)| *score >= -50)
            .sorted_by(|(_, s1), (_, s2)| s1.cmp(s2))
            .for_each(|p| {
                println!("{:?}", p);
            });
    }

    #[test]
    fn find_every_steel_complement() {
        use BasicType::*;
        Pokemon::all()
            .into_iter()
            .filter(|p| p.typing.contains(Steel))
            .unique()
            .for_each(|poke| {
                println!("{poke:?}");
                Pokemon::all()
                    .into_iter()
                    .unique()
                    .map(|p| (p.clone(), resistance_complements(&poke, &p)))
                    .max_set_by_key(|(_, s)| *s)
                    .into_iter()
                    .for_each(|(p, s)| {
                        println!("  {p:?} {s:?}");
                    });
            });
    }

}