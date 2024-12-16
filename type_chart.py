"""
Generation 6+ Pokemon Type Chart
"""
from __future__ import annotations
from dataclasses import dataclass, field
from enum import Enum, auto
from functools import cache
from typing import Iterable, TypeVar, overload

T = TypeVar("T")

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

    LEVITATE = auto()

    def __repr__(self):
        return f"Type.{self.name}"

    def __str__(self):
        return self.name

    @overload
    @staticmethod
    def natural_order(value: dict[Type, T]) -> dict[Type, T]:
        ...

    @overload
    @staticmethod
    def natural_order(value: Iterable[Type]) -> list[Type]:
        ...

    @staticmethod
    def natural_order(value):
        if isinstance(value, dict):
            ret = {}
            for t in Type:
                if t in value:
                    ret[t] = value[t]
            return ret
        elif isinstance(value, Iterable):
            return sorted(value, key=lambda x: x.name)
        raise TypeError(f"Unsupported type {type(value)}")
        
    def __str__(self):
        return self.name.capitalize()
    
    @classmethod
    def from_str(cls, value: str) -> Type:
        return cls[value.upper()]
    
    @staticmethod
    def basic() -> list[Type]:
        return [Type.NORMAL, Type.FIRE, Type.WATER, Type.ELECTRIC, Type.GRASS, Type.ICE, Type.FIGHTING, Type.POISON, Type.GROUND, Type.FLYING, Type.PSYCHIC, Type.BUG, Type.ROCK, Type.GHOST, Type.DRAGON, Type.DARK, Type.STEEL, Type.FAIRY]

@dataclass(frozen=True)
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
        no_effect={Type.FLYING, Type.LEVITATE},
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
    Type.LEVITATE: TypeRelationship(
    ),
}