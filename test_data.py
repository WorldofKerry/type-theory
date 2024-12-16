from data import REAL_POKEMON_TYPES
from type_chart import Type

def test_data():
    for pokemon in REAL_POKEMON_TYPES:
        if Type.LEVITATE in pokemon._types:
            print(pokemon)