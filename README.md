# Pokemon Typing Analysis

## Flamegraph
```bash
cargo install flamegraph
apt install linux-tools-generic
PERF=/usr/lib/linux-tools/5.15.0-127-generic/perf CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph --bin=auto-team --no-inline
```

## Data Source
- [pokemon_data.csv](https://www.kaggle.com/datasets/guavocado/pokemon-stats-1025-pokemons)
- [metadata_pokemon_moves.csv](https://www.kaggle.com/datasets/thiagoamancio/full-pokemons-and-moves-datasets?select=metadata_pokemon_moves.csv)
