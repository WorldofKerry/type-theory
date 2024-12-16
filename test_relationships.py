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
