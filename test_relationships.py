from collections import defaultdict
import itertools
from statistics import geometric_mean
from typing import Optional

import pytest
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

def test_best_resistances():
    # teams with each member being mono-type
    count_to_team = defaultdict(set)
    
    for types in itertools.combinations(Type, 2):
        team = Team.from_list(MultiType(t) for t in types)
        resistances = team.resistances_count()
        count_to_team[len(resistances)].add(team)

    count_to_team = dict(sorted(count_to_team.items(), reverse=True))
    
    assert max(count_to_team.keys()) == 14
    assert count_to_team[14] == {Team(MultiType(Type.GRASS), MultiType(Type.STEEL)), Team(MultiType(Type.DRAGON), MultiType(Type.STEEL))}

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

        resist_count = len(team.resistances_count())
        avg = team.average_damage(immunity_multiplier=0.25)

        teams.append((resist_count, avg, team))

    for team in sorted(teams, key=lambda x: (x[1]), reverse=True):
        print(team)

def test_good_team():
    # team = Team(MultiType(Type.FIRE, Type.FLYING), MultiType(Type.ELECTRIC, Type.STEEL), MultiType(Type.DRAGON, Type.FIGHTING))
    # team = Team(MultiType(Type.GRASS, Type.PSYCHIC), MultiType(Type.DRAGON, Type.PSYCHIC), MultiType(Type.GRASS, Type.ICE))
    # team = Team(MultiType(Type.FAIRY, Type.LEVITATE, Type.POISON), MultiType(Type.DRAGON, Type.STEEL))
    team = Team(MultiType(Type.ELECTRIC))
    print(evaluate_team(team))

def compute_product_damage_per_type(team: Team, immunity_multiplier: float) -> dict[Type, float]:
    """
    E.g. is a 4x weakness to a type complemented by two 2x resistances or a 4x resistance?
    """
    member_defences = [member.defense() for member in team._members]
    type_damages = dict.fromkeys(Type, 1.0)
    for type in Type:
        for member_defence in member_defences:
            type_damages[type] *= member_defence[type] if member_defence[type] != 0.0 else immunity_multiplier
    return type_damages

def compute_weakness_product(team: Team, immunity_multiplier: float = 0.25) -> float:
    type_damages = compute_product_damage_per_type(team, immunity_multiplier)
    weakness_product = 1.0
    for td in type_damages.values():
        if td > 1:
            weakness_product *= td
    return weakness_product

def compute_missing_resistance_coverage(team: Team):
    """
    Compute how many weaknesses are not covered by resistances
    """
    weaknesses = team.weaknesses_count()
    resistances = team.resistances_count()
    missed_count = 0
    for weakness, weakness_count in weaknesses.items():
        if weakness not in resistances or resistances[weakness] < weakness_count:
            missed_count += 1
    return missed_count

def compute_offensive_coverage(team: Team) -> float:
    """
    Compute how well each member is able to attack the other types
    """
    net_coverage = 0.0
    for member in team._members:
        net_coverage += member.attack_coverage(REAL_POKEMON_TYPES).filter(Effectiveness.MORE_EFFECTIVE).__len__()
        net_coverage -= member.attack_coverage(REAL_POKEMON_TYPES).filter(Effectiveness.LESS_EFFECTIVE).__len__()
    return net_coverage

def evaluate_team(team: Team) -> tuple[float, ...]:
    missing_resistance_coverage = compute_missing_resistance_coverage(team)
    weakness_product = compute_weakness_product(team)
    # resisted_count = len(team.resistances_count())
    # avg_dmg = team.average_damage(immunity_multiplier=0.25)
    net_coverage = compute_offensive_coverage(team)
    return (missing_resistance_coverage, weakness_product, -net_coverage, team)

@pytest.mark.skip
def test_find_best_team():
    entries = []
    
    for types in itertools.combinations(REAL_POKEMON_TYPES, 3):
        team = Team.from_list(types)
        entries.append(evaluate_team(team))

    for team in sorted(entries, key=lambda x: x[:-1], reverse=True):
        print(repr(team))
