"""
Parses Smogon / Pokemon Showdown team files
"""

from pokeapi_api import get_type
from relationships import MultiType, Team

def parse_team_file(file_path: str) -> list[MultiType]:
    with open(file_path) as f:
        lines = f.readlines()

    member_types = list()
    prev_blank = True
    for line in lines:
        if prev_blank:
            prev_blank = False
            pokemon_name = line.strip().split(" ")[0]
            member_types.append(get_type(pokemon_name))
        if line == "\n":
            prev_blank = True

    return member_types