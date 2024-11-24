from complex_relationships import MultiType, Type

def test_all_types():
    assert len(MultiType.all_types()) == len(Type)
    assert len(MultiType.all_types(2)) == len(Type) * (len(Type) - 1) // 2

def test_attack_type_coverage():
    defense = MultiType.all_types(2) | MultiType.all_types(1)
    relationship = MultiType(Type.NORMAL, Type.GHOST).attack_coverage(defense)

    filtered_relationship = {k: v for k, v in relationship.items() if v < 1.0}
    assert filtered_relationship == {
        MultiType(Type.NORMAL, Type.GHOST): 0.0,
        MultiType(Type.STEEL, Type.NORMAL): 0.5,
        MultiType(Type.DARK, Type.STEEL): 0.5,
        MultiType(Type.DARK, Type.ROCK): 0.5,
        MultiType(Type.ROCK, Type.NORMAL): 0.5,
    }

