from collections import defaultdict
import csv
from functools import cache
from type_chart import Type
from relationships import MultiType

def _parse_types() -> dict[int, Type]:
    types = {}
    reader = csv.reader(open("data/types.csv"))
    next(reader)
    for row in reader:
        type_id, type_name = row[:2]
        try:
            types[int(type_id)] = Type.from_str(type_name)
        except KeyError:
            assert type_name in ["unknown", "shadow"], f"Unknown type {type_id=}"
    return types

def _parse_abilities() -> dict[int, str]:
    abilities = {}
    reader = csv.reader(open("data/abilities.csv"))
    next(reader)
    for row in reader:
        pokemon_id, ability_id = row[:2]
        abilities[int(pokemon_id)] = ability_id
    return abilities

@cache
def get_all_pokemon_multitypes() -> set[MultiType]:
    types = _parse_types()
    abilities = _parse_abilities()
    pokemon: dict[int, set[Type]] = defaultdict(set) # keyed by pokemon id
    
    reader = csv.reader(open("data/pokemon_types.csv"))
    next(reader)
    for row in reader:
        pokemon_id, type_id = row[:2]
        pokemon[int(pokemon_id)].add(types[int(type_id)])

    reader = csv.reader(open("data/pokemon_abilities.csv"))
    next(reader)
    for row in reader:
        pokemon_id, ability_id = row[:2]
        try:
            pokemon[int(pokemon_id)].add(Type.from_str(abilities[int(ability_id)]))
        except KeyError:
            pass

    return set(MultiType(*types) for types in pokemon.values())

REAL_POKEMON_TYPES: set[MultiType] = get_all_pokemon_multitypes()
