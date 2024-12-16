"""
Parses Smogon / Pokemon Showdown team files
"""

from pokeapi_api import get_type
from relationships import Team

def parse_team_file(file_path: str) -> Team:
    with open(file_path) as f:
        lines = f.readlines()

    member_types = set()
    prev_blank = True
    for line in lines:
        if prev_blank:
            prev_blank = False
            pokemon_name = line.strip().split(" ")[0]
            member_types.add(get_type(pokemon_name))
        if line == "\n":
            prev_blank = True

    return Team.from_list(list(member_types))