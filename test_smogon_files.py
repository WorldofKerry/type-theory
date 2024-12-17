import itertools
from relationships import Team
from smogon_files import parse_team, parse_team_file
from test_relationships import evaluate_team
from type_chart import Type

def test_parse_team_file():
    types = parse_team_file("team.txt")

    entries = set()
    
    for types in itertools.combinations(types, 6):
        team = Team.from_list(types)
        entries.add(evaluate_team(team))

    for team in sorted(entries, key=lambda x: x[:-1], reverse=True):
        print(repr(team))

def test_parse_team_txt():
    txt = """
Rotom @ Leftovers
IVs: 7 HP / 24 Def / 0 SpA / 23 SpD / 28 Spe
EVs: 1 HP / 10 Atk / 12 Def / 9 SpA / 4 SpD / 8 Spe
Ability: Levitate
Level: 34
Docile Nature
- Ominous Wind
- Uproar
- Double Team
- Shock Wave

Ariados (M) @ Quick Claw
IVs: 15 HP / 24 Atk / 30 Def / 20 SpA / 3 SpD / 5 Spe
EVs: 7 HP / 7 Atk / 5 Def / 10 SpA / 7 SpD / 17 Spe
Ability: Swarm
Level: 31
Mild Nature
- Venoshock
- Shadow Sneak
- Leech Life
- Thief
"""
    types = parse_team(txt)
    team = Team.from_list(types)
    
    assert team.weaknesses_count().keys() == {Type.FIRE, Type.FLYING, Type.PSYCHIC, Type.ROCK, Type.GHOST, Type.DARK}
