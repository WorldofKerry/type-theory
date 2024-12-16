import itertools
from relationships import Team
from smogon_files import parse_team, parse_team_file

def test_parse_team_file():
    types = parse_team_file("team.txt")

    teams = set()

    for ts in itertools.combinations(types, 6):
        team = Team.from_list(ts)
        weaknesses = team.weaknesses_count()
        resistances = team.resistances_count()
        for weakness, count in weaknesses.items():
            if weakness not in resistances or count > resistances[weakness] + 1:
                break
        else:
            teams.add(team)

    for team in teams:
        print(team)

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
    team = parse_team(txt)
    print(team)