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
class TypeRelationship:
    no_effect: set[Type] = field(default_factory=set)
    half_effective: set[Type] = field(default_factory=set)
    double_effective: set[Type] = field(default_factory=set)

ATTACK_TYPE_CHART = {
    Type.NORMAL: TypeRelationship(
        no_effect={Type.GHOST},
        half_effective={Type.ROCK, Type.STEEL},
    ),
    Type.FIRE: TypeRelationship(
        half_effective={Type.FIRE, Type.WATER, Type.ROCK, Type.DRAGON},
        double_effective={Type.GRASS, Type.ICE, Type.BUG, Type.STEEL},
    ),
    Type.WATER: TypeRelationship(
        half_effective={Type.WATER, Type.GRASS, Type.DRAGON},
        double_effective={Type.FIRE, Type.GROUND, Type.ROCK},
    ),
    Type.ELECTRIC: TypeRelationship(
        no_effect={Type.GROUND},
        half_effective={Type.ELECTRIC, Type.GRASS, Type.DRAGON},
        double_effective={Type.WATER, Type.FLYING},
    ),
    Type.GRASS: TypeRelationship(
        half_effective={Type.FIRE, Type.GRASS, Type.POISON, Type.FLYING, Type.BUG, Type.DRAGON, Type.STEEL},
        double_effective={Type.WATER, Type.GROUND, Type.ROCK},
    ),
    Type.ICE: TypeRelationship(
        half_effective={Type.FIRE, Type.WATER, Type.ICE, Type.STEEL},
        double_effective={Type.GRASS, Type.GROUND, Type.FLYING, Type.DRAGON},
    ),
    Type.FIGHTING: TypeRelationship(
        no_effect={Type.GHOST},
        half_effective={Type.POISON, Type.FLYING, Type.PSYCHIC, Type.BUG, Type.FAIRY},
        double_effective={Type.NORMAL, Type.ICE, Type.ROCK, Type.DARK, Type.STEEL},
    ),
    Type.POISON: TypeRelationship(
        half_effective={Type.POISON, Type.GROUND, Type.ROCK, Type.GHOST},
        double_effective={Type.GRASS, Type.FAIRY},
    ),
    Type.GROUND: TypeRelationship(
        no_effect={Type.FLYING},
        half_effective={Type.GRASS, Type.BUG},
        double_effective={Type.FIRE, Type.ELECTRIC, Type.POISON, Type.ROCK, Type.STEEL},
    ),
    Type.FLYING: TypeRelationship(
        half_effective={Type.ELECTRIC, Type.ROCK, Type.STEEL},
        double_effective={Type.GRASS, Type.FIGHTING, Type.BUG},
    ),
    Type.PSYCHIC: TypeRelationship(
        no_effect={Type.DARK},
        half_effective={Type.PSYCHIC, Type.STEEL},
        double_effective={Type.FIGHTING, Type.POISON},
    ),
    Type.BUG: TypeRelationship(
        half_effective={Type.FIRE, Type.FIGHTING, Type.POISON, Type.FLYING, Type.GHOST, Type.STEEL, Type.FAIRY},
        double_effective={Type.GRASS, Type.PSYCHIC, Type.DARK},
    ),
    Type.ROCK: TypeRelationship(
        half_effective={Type.FIGHTING, Type.GROUND, Type.STEEL},
        double_effective={Type.FIRE, Type.ICE, Type.FLYING, Type.BUG},
    ),
    Type.GHOST: TypeRelationship(
        no_effect={Type.NORMAL},
        half_effective={Type.DARK},
        double_effective={Type.PSYCHIC, Type.GHOST},
    ),
    Type.DRAGON: TypeRelationship(
        no_effect={Type.FAIRY},
        half_effective={Type.STEEL},
        double_effective={Type.DRAGON},
    ),
    Type.DARK: TypeRelationship(
        half_effective={Type.FIGHTING, Type.DARK, Type.FAIRY},
        double_effective={Type.PSYCHIC, Type.GHOST},
    ),
    Type.STEEL: TypeRelationship(
        half_effective={Type.FIRE, Type.WATER, Type.ELECTRIC, Type.STEEL},
        double_effective={Type.ICE, Type.ROCK, Type.FAIRY},
    ),
    Type.FAIRY: TypeRelationship(
        half_effective={Type.FIRE, Type.POISON, Type.STEEL},
        double_effective={Type.FIGHTING, Type.DRAGON, Type.DARK},
    ),
}