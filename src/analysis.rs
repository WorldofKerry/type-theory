use crate::pokemon::Pokemon;
use rayon::prelude::*;

#[cfg(test)]
mod test {

    use std::collections::HashMap;

    use strum::IntoEnumIterator;

    use crate::{pokemon::Typing, typing::{Ability, BasicType, Relationship, TypeTrait}};

    use super::*;

    #[test]
    fn get_best_defensive_pokemon() {
        let pokemon: Vec<Pokemon> = Pokemon::all().collect();

        // Get all of their type charts, sped up with rayon
        let defense_charts: HashMap<Pokemon, Relationship> = pokemon.into_par_iter().map(|p| (p.clone(), p.defense())).collect();

        println!("{:?}", defense_charts[&Pokemon { typing: Typing::Dual(BasicType::Dark, BasicType::Dragon), ability: Some(Ability::Levitate) }]);
    }
}