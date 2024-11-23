from complex_relationships import MultiType, Type

def test_all_types():
    assert len(MultiType.all_types()) == len(Type)
    assert len(MultiType.all_types(2)) == len(Type) * (len(Type) - 1) // 2
