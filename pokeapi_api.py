"""
Pokeapi
"""

import requests
from relationships import MultiType
from type_chart import Type


def get_type(name: str) -> MultiType:
    # Gets the type from the pokemon name
    
    url = f"https://pokeapi.co/api/v2/pokemon/{name.lower()}"
    response = requests.get(url)
    response.raise_for_status()
    data = response.json()
    
    ts = set()
    for t in data['types']:
        ts.add(Type.from_str(t['type']['name']))
    return MultiType(*ts)
