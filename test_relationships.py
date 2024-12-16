from collections import defaultdict
import itertools
from data import REAL_POKEMON_TYPES
from relationships import MultiType, Effectiveness, Team, Type

def assert_weakness_count(*args, count: int):
    relationships = MultiType(*args).defense()
    assert len(relationships.filter(Effectiveness.MORE_EFFECTIVE)) == count, f"{args=} {relationships=}"

def test_assert_one_weaknesses():
    combos = [
        [Type.NORMAL],
        [Type.ELECTRIC],
        [Type.NORMAL, Type.GHOST],
        [Type.WATER, Type.GROUND],
        [Type.POISON, Type.DARK],
        [Type.BUG, Type.STEEL],
        [Type.GHOST, Type.DARK],
    ]
    for combo in combos:
        assert_weakness_count(*combo, count=1)

def test_seven_weaknesses():
    combos = [
        [Type.GRASS, Type.ICE],
        [Type.GRASS, Type.PSYCHIC],
        [Type.GRASS, Type.DARK],
        [Type.FIGHTING, Type.ROCK],
        [Type.PSYCHIC, Type.ROCK],
        [Type.ROCK, Type.DARK],
    ]
    for combo in combos:
        assert_weakness_count(*combo, count=7)

def test_select_weaknesses():
    assert_weakness_count(Type.FIRE, Type.BUG, count=3)
    assert_weakness_count(Type.ELECTRIC, Type.FLYING, count=2)
    assert_weakness_count(Type.ELECTRIC, Type.GRASS, count=4)
    assert_weakness_count(Type.DRAGON, Type.FAIRY, count=4)

def test_immunities():
    assert MultiType(Type.GRASS, Type.FAIRY).defense().filter(Effectiveness.NO_EFFECT).keys() == {Type.DRAGON}

def test_team_1():
    team = Team(
        MultiType(Type.WATER),
        MultiType(Type.GRASS),
    )

    assert team.weaknesses_count() == {Type.FIRE: 1, Type.ELECTRIC: 1, Type.GRASS: 1, Type.ICE: 1, Type.POISON: 1, Type.FLYING: 1, Type.BUG: 1}

    assert team.resistances_count() == {Type.FIRE: 1, Type.WATER: 2, Type.ELECTRIC: 1, Type.GRASS: 1, Type.ICE: 1, Type.GROUND: 1, Type.STEEL: 1}

def test_find_best_resistances():
    # teams with each member being mono-type
    count_to_team = defaultdict(set)
    
    for types in itertools.combinations(Type, 2):
        team = Team.from_list(MultiType(t) for t in types)
        resistances = team.resistances_count()
        count_to_team[len(resistances)].add(team)

    count_to_team = dict(sorted(count_to_team.items(), reverse=True))
    
    assert max(count_to_team.keys()) == 13
    print(count_to_team[13])
    assert count_to_team[13] == {Team(MultiType(Type.GRASS), MultiType(Type.STEEL)), Team(MultiType(Type.DRAGON), MultiType(Type.STEEL))}

def test_find_complementary_monotype_team():
    teams = set()

    for types in itertools.combinations(Type, 3):
        team = Team.from_list(MultiType(t) for t in types)
        weaknesses = team.weaknesses_count()
        resistances = team.resistances_count()
        for weakness in weaknesses:
            if weakness not in resistances:
                break
        else:
            teams.add(team)

    print(teams)

def test_find_complementary_team():
    teams = []

    for types in itertools.combinations(REAL_POKEMON_TYPES, 2):
        team = Team.from_list(types)
        weaknesses = team.weaknesses_count()
        resistances = team.resistances_count()
        for weakness, weak_count in weaknesses.items():
            if weakness not in resistances:
                break
        else:
            teams.append((sum(resistances.values()), team))

    for team in sorted(teams, key=lambda x: x[0]):
        print(team)

def test_find_product_weaknesses_resistances():
    teams = []

    for types in itertools.combinations(REAL_POKEMON_TYPES, 2):
        team = Team.from_list(types)
        product = team.product_weaknesses_resistances(immune_weight=1)
        resist_count = len(team.resistances_count())
        teams.append((-resist_count, sum(product.values()), team))

    for team in sorted(teams, key=lambda x: x[0], reverse=True):
        print(team)

def test_good_team():
    team = Team.from_list((MultiType(Type.STEEL, Type.GHOST), MultiType(Type.LEVITATE, Type.DRAGON, Type.DARK)))
    print(team.resistances_count())
