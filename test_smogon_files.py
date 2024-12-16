import itertools
from relationships import Team
from smogon_files import parse_team_file

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
