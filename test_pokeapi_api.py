from pokeapi_api import get_type
from relationships import MultiType
from type_chart import Type

def test_get_type():
    assert get_type("skarmory") == MultiType(Type.STEEL, Type.FLYING)
