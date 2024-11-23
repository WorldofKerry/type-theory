"""
Generation 6+ Pokemon Type Chart
"""

from dataclasses import dataclass, field
from enum import Enum, auto
from functools import cache
from typing import Optional

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
    quarter_effective: set[Type] = field(default_factory=set)
    half_effective: set[Type] = field(default_factory=set)
    double_effective: set[Type] = field(default_factory=set)
    quad_effective: set[Type] = field(default_factory=set)

    @property
    def normal_effective(self) -> set[Type]:
        return set(Type) - self.no_effect - self.half_effective - self.double_effective

    @property
    def less_effective(self) -> set[Type]:
        return self.half_effective | self.quarter_effective | self.no_effect

    @property
    def more_effective(self) -> set[Type]:
        return self.double_effective | self.quad_effective

    @classmethod
    @cache
    def from_types(cls, type1: Type, type2: Optional[Type] = None):
        """
        Relationship of a dual type defending pokemon to an attacking type
        """
        type_multipliers = dict.fromkeys(Type, 1.0)
        for attack_type, relationship in ATTACK_TYPE_CHART.items():
            type_multipliers[attack_type] *= 0.0 if type1 in relationship.no_effect else 1.0
            type_multipliers[attack_type] *= 0.5 if type1 in relationship.half_effective else 1.0
            type_multipliers[attack_type] *= 2.0 if type1 in relationship.double_effective else 1.0
            if type2 is not None:
                type_multipliers[attack_type] *= 0.0 if type2 in relationship.no_effect else 1.0
                type_multipliers[attack_type] *= 0.5 if type2 in relationship.half_effective else 1.0
                type_multipliers[attack_type] *= 2.0 if type2 in relationship.double_effective else 1.0
        return cls(
            no_effect={attack_type for attack_type, multiplier in type_multipliers.items() if multiplier == 0.0},
            quarter_effective={attack_type for attack_type, multiplier in type_multipliers.items() if multiplier == 0.25},
            half_effective={attack_type for attack_type, multiplier in type_multipliers.items() if multiplier == 0.5},
            double_effective={attack_type for attack_type, multiplier in type_multipliers.items() if multiplier == 2.0},
            quad_effective={attack_type for attack_type, multiplier in type_multipliers.items() if multiplier == 4.0},
        )

@dataclass
class FullType:
    types: set[Type]

def best_coverage(*types: Type) -> set[FullType]:
    """
    Given move types, 
    """

ATTACK_TYPE_CHART = {
    Type.NORMAL: Relationship(
        no_effect={Type.GHOST},
        half_effective={Type.ROCK, Type.STEEL},
    ),
    Type.FIRE: Relationship(
        half_effective={Type.FIRE, Type.WATER, Type.ROCK, Type.DRAGON},
        double_effective={Type.GRASS, Type.ICE, Type.BUG, Type.STEEL},
    ),
    Type.WATER: Relationship(
        half_effective={Type.WATER, Type.GRASS, Type.DRAGON},
        double_effective={Type.FIRE, Type.GROUND, Type.ROCK},
    ),
    Type.ELECTRIC: Relationship(
        no_effect={Type.GROUND},
        half_effective={Type.ELECTRIC, Type.GRASS, Type.DRAGON},
        double_effective={Type.WATER, Type.FLYING},
    ),
    Type.GRASS: Relationship(
        half_effective={Type.FIRE, Type.GRASS, Type.POISON, Type.FLYING, Type.BUG, Type.DRAGON, Type.STEEL},
        double_effective={Type.WATER, Type.GROUND, Type.ROCK},
    ),
    Type.ICE: Relationship(
        half_effective={Type.FIRE, Type.WATER, Type.ICE, Type.STEEL},
        double_effective={Type.GRASS, Type.GROUND, Type.FLYING, Type.DRAGON},
    ),
    Type.FIGHTING: Relationship(
        no_effect={Type.GHOST},
        half_effective={Type.POISON, Type.FLYING, Type.PSYCHIC, Type.BUG, Type.FAIRY},
        double_effective={Type.NORMAL, Type.ICE, Type.ROCK, Type.DARK, Type.STEEL},
    ),
    Type.POISON: Relationship(
        half_effective={Type.POISON, Type.GROUND, Type.ROCK, Type.GHOST},
        double_effective={Type.GRASS, Type.FAIRY},
    ),
    Type.GROUND: Relationship(
        no_effect={Type.FLYING},
        half_effective={Type.GRASS, Type.BUG},
        double_effective={Type.FIRE, Type.ELECTRIC, Type.POISON, Type.ROCK, Type.STEEL},
    ),
    Type.FLYING: Relationship(
        half_effective={Type.ELECTRIC, Type.ROCK, Type.STEEL},
        double_effective={Type.GRASS, Type.FIGHTING, Type.BUG},
    ),
    Type.PSYCHIC: Relationship(
        no_effect={Type.DARK},
        half_effective={Type.PSYCHIC, Type.STEEL},
        double_effective={Type.FIGHTING, Type.POISON},
    ),
    Type.BUG: Relationship(
        half_effective={Type.FIRE, Type.FIGHTING, Type.POISON, Type.FLYING, Type.GHOST, Type.STEEL, Type.FAIRY},
        double_effective={Type.GRASS, Type.PSYCHIC, Type.DARK},
    ),
    Type.ROCK: Relationship(
        half_effective={Type.FIGHTING, Type.GROUND, Type.STEEL},
        double_effective={Type.FIRE, Type.ICE, Type.FLYING, Type.BUG},
    ),
    Type.GHOST: Relationship(
        no_effect={Type.NORMAL},
        half_effective={Type.DARK},
        double_effective={Type.PSYCHIC, Type.GHOST},
    ),
    Type.DRAGON: Relationship(
        no_effect={Type.FAIRY},
        half_effective={Type.STEEL},
        double_effective={Type.DRAGON},
    ),
    Type.DARK: Relationship(
        half_effective={Type.FIGHTING, Type.DARK, Type.FAIRY},
        double_effective={Type.PSYCHIC, Type.GHOST},
    ),
    Type.STEEL: Relationship(
        half_effective={Type.FIRE, Type.WATER, Type.ELECTRIC, Type.STEEL},
        double_effective={Type.ICE, Type.ROCK, Type.FAIRY},
    ),
    Type.FAIRY: Relationship(
        half_effective={Type.FIRE, Type.POISON, Type.STEEL},
        double_effective={Type.FIGHTING, Type.DRAGON, Type.DARK},
    ),
}

def main():
    print(f"{Relationship.from_types(Type.NORMAL)=}")
    print(f"{Relationship.from_types(Type.ELECTRIC)=}")
    print(f"{Relationship.from_types(Type.NORMAL, Type.GHOST)=}")

if __name__ == "__main__":
    main()
