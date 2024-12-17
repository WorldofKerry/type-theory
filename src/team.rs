use crate::pokemon::Pokemon;
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Team {
    pub pokemon: Vec<Pokemon>,
}

impl Team {
    pub fn all(pool: impl Iterator<Item = Pokemon>, size: usize) -> impl Iterator<Item = Team> {
        pool.combinations(size).map(move |team| Team { pokemon: team.into_iter().map(|p| p.clone()).collect()})
    }

    pub fn random(pool: impl Iterator<Item = Pokemon>, size: usize) -> Team {
        let pool = pool.collect::<Vec<_>>();
        let team = (0..size).map(|_| Pokemon::random(&pool)).collect();
        Team { pokemon: team }
    }

    pub fn fill_random(&self, pool: impl Iterator<Item = Pokemon>, size: usize) -> Team {
        let missing = size - self.pokemon.len();
        Team { pokemon: Team::random(pool, missing).pokemon.into_iter().chain(self.pokemon.iter().map(|p| p.clone())).collect() }
    }
}
