from relationships import Relationship, Type

def test_one_weakness():
    def single(*args):
        relationship = Relationship.from_types(*args)
        print(f"{args=} {relationship=}")
        print(f"{relationship.super_effective=}")
        assert len(relationship.super_effective) == 1, f"{args=} {relationship=}"

    single(Type.NORMAL)
    single(Type.ELECTRIC)
    single(Type.NORMAL, Type.GHOST)
    single(Type.WATER, Type.GROUND)
