use crate::pokemon::Pokemon;

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn get_best_defensive_pokemon() {
        let pokemon: Vec<Pokemon> = Pokemon::all().collect();
        println!("{:?}", pokemon);
    }
}