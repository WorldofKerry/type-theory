"""
Parses Smogon / Pokemon Showdown team files
"""

from pokeapi_api import get_type
from relationships import MultiType, Team
from type_chart import Type

def parse_team_file(file_path: str) -> list[MultiType]:
    with open(file_path) as f:
        return parse_team(f.read())

def parse_team(team: str) -> list[MultiType]:
    lines = team.split("\n")
    
    while lines[0].strip() == "":
        lines = lines[1:]

    member_types = list()
    prev_blank = True

    current_types = set()

    for line in lines:
        # Add pokemon types by name
        if prev_blank:
            prev_blank = False
            pokemon_name = line.strip().split(" ")[0]
            current_types.update(get_type(pokemon_name))

        # Add ability with type interaction
        if line.startswith("Ability: "):
            ability = line.strip()[len("Ability: "):]
            try:
                current_types.add(Type.from_str(ability))
            except KeyError:
                pass

        # Delimiter for next pokemon
        if line.strip() == "":
            assert current_types, "No types found for pokemon"
            member_types.append(MultiType(*current_types))
            current_types = set()
            prev_blank = True

    if current_types:
        member_types.append(MultiType(*current_types))

    return member_types