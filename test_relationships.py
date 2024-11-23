from relationships import Relationship, Type

def assert_weakness_count(*args, count: int):
    relationship = Relationship.from_types(*args)
    assert len(relationship.super_effective) == count, f"{args=} {relationship=}"

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
