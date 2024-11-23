from complex_relationships import MultiType, Type

def test_all_types():
    assert len(MultiType.all_types()) == len(Type)
    assert len(MultiType.all_types(2)) == len(Type) * (len(Type) - 1) // 2

def test_attack_type_coverage():
    defense = MultiType.all_types(2) | MultiType.all_types(1)
    attack = MultiType(Type.NORMAL, Type.GHOST)

    relationship = attack.attack(defense)

    filtered_relationship = {k: v for k, v in relationship.items() if v < 1.0}
    print(filtered_relationship)
    assert False
