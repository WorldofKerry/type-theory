from smogon_files import parse_team_file

def test_parse_team_file():
    team = parse_team_file("team.txt")
    print(team)