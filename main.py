"""
Generation 6+ Pokemon Type Chart
"""

from dataclasses import dataclass, field
from enum import Enum, auto
from functools import cache

class Type(Enum):
    NORMAL = auto()
    FIRE = auto()
    WATER = auto()
    ELECTRIC = auto()
    GRASS = auto()
    ICE = auto()
    FIGHTING = auto()
    POISON = auto()
    GROUND = auto()
    FLYING = auto()
    PSYCHIC = auto()
    BUG = auto()
    ROCK = auto()
    GHOST = auto()
    DRAGON = auto()
    DARK = auto()
    STEEL = auto()
    FAIRY = auto()

@dataclass
class Relationship:
    no_effect: set[Type] = field(default_factory=set)
    not_very_effective: set[Type] = field(default_factory=set)
    super_effective: set[Type] = field(default_factory=set)

    @property
    def normal_effective(self):
        return set(Type) - self.no_effect - self.not_very_effective - self.super_effective

ATTACK_TYPE_CHART = {
    Type.NORMAL: Relationship(
        no_effect={Type.GHOST},
        not_very_effective={Type.ROCK, Type.STEEL},
    ),
    Type.FIRE: Relationship(
        not_very_effective={Type.FIRE, Type.WATER, Type.ROCK, Type.DRAGON},
        super_effective={Type.GRASS, Type.ICE, Type.BUG, Type.STEEL},
    ),
    Type.WATER: Relationship(
        not_very_effective={Type.WATER, Type.GRASS, Type.DRAGON},
        super_effective={Type.FIRE, Type.GROUND, Type.ROCK},
    ),
    Type.ELECTRIC: Relationship(
        no_effect={Type.GROUND},
        not_very_effective={Type.ELECTRIC, Type.GRASS, Type.DRAGON},
        super_effective={Type.WATER, Type.FLYING},
    ),
    Type.GRASS: Relationship(
        not_very_effective={Type.FIRE, Type.GRASS, Type.POISON, Type.FLYING, Type.BUG, Type.DRAGON, Type.STEEL},
        super_effective={Type.WATER, Type.GROUND, Type.ROCK},
    ),
    Type.ICE: Relationship(
        not_very_effective={Type.FIRE, Type.WATER, Type.ICE, Type.STEEL},
        super_effective={Type.GRASS, Type.GROUND, Type.FLYING, Type.DRAGON},
    ),
    Type.FIGHTING: Relationship(
        no_effect={Type.GHOST},
        not_very_effective={Type.POISON, Type.FLYING, Type.PSYCHIC, Type.BUG, Type.FAIRY},
        super_effective={Type.NORMAL, Type.ICE, Type.ROCK, Type.DARK, Type.STEEL},
    ),
    Type.POISON: Relationship(
        not_very_effective={Type.POISON, Type.GROUND, Type.ROCK, Type.GHOST},
        super_effective={Type.GRASS, Type.FAIRY},
    ),
    Type.GROUND: Relationship(
        no_effect={Type.FLYING},
        not_very_effective={Type.GRASS, Type.BUG},
        super_effective={Type.FIRE, Type.ELECTRIC, Type.POISON, Type.ROCK, Type.STEEL},
    ),
    Type.FLYING: Relationship(
        not_very_effective={Type.ELECTRIC, Type.ROCK, Type.STEEL},
        super_effective={Type.GRASS, Type.FIGHTING, Type.BUG},
    ),
    Type.PSYCHIC: Relationship(
        not_very_effective={Type.PSYCHIC, Type.STEEL},
        super_effective={Type.FIGHTING, Type.POISON},
    ),
    Type.BUG: Relationship(
        not_very_effective={Type.FIRE, Type.FIGHTING, Type.POISON, Type.FLYING, Type.GHOST, Type.STEEL, Type.FAIRY},
        super_effective={Type.GRASS, Type.PSYCHIC, Type.DARK},
    ),
    Type.ROCK: Relationship(
        not_very_effective={Type.FIGHTING, Type.GROUND, Type.STEEL},
        super_effective={Type.FIRE, Type.ICE, Type.FLYING, Type.BUG},
    ),
    Type.GHOST: Relationship(
        no_effect={Type.NORMAL},
        not_very_effective={Type.DARK},
        super_effective={Type.GHOST, Type.DARK},
    ),
    Type.DRAGON: Relationship(
        no_effect={Type.FAIRY},
        not_very_effective={Type.STEEL},
        super_effective={Type.DRAGON},
    ),
    Type.DARK: Relationship(
        not_very_effective={Type.FIGHTING, Type.DARK, Type.FAIRY},
        super_effective={Type.PSYCHIC, Type.GHOST},
    ),
    Type.STEEL: Relationship(
        not_very_effective={Type.FIRE, Type.WATER, Type.ELECTRIC, Type.STEEL},
        super_effective={Type.ICE, Type.ROCK, Type.FAIRY},
    ),
    Type.FAIRY: Relationship(
        not_very_effective={Type.FIRE, Type.POISON, Type.STEEL},
        super_effective={Type.FIGHTING, Type.DRAGON, Type.DARK},
    ),
}

@cache
def get_relationship(type1: Type, type2: Type = None) -> set[Type]:
    type_multipliers = dict.fromkeys(Type, 1.0)
    for attack_type, relationship in ATTACK_TYPE_CHART.items():
        type_multipliers[attack_type] *= 0.0 if type1 in relationship.no_effect else 1.0
        type_multipliers[attack_type] *= 0.5 if type1 in relationship.not_very_effective else 1.0
        type_multipliers[attack_type] *= 2.0 if type1 in relationship.super_effective else 1.0
        if type2 is not None:
            type_multipliers[attack_type] *= 0.0 if type2 in relationship.no_effect else 1.0
            type_multipliers[attack_type] *= 0.5 if type2 in relationship.not_very_effective else 1.0
            type_multipliers[attack_type] *= 2.0 if type2 in relationship.super_effective else 1.0
    return Relationship(
        no_effect={attack_type for attack_type, multiplier in type_multipliers.items() if multiplier == 0.0},
        not_very_effective={attack_type for attack_type, multiplier in type_multipliers.items() if multiplier == 0.5},
        super_effective={attack_type for attack_type, multiplier in type_multipliers.items() if multiplier == 2.0},
    )

def main():
    print(f"{get_relationship(Type.NORMAL)=}")
    print(f"{get_relationship(Type.ELECTRIC)=}")
    print(f"{get_relationship(Type.NORMAL, Type.GHOST)=}")

if __name__ == "__main__":
    main()
