from __future__ import annotations
from dataclasses import dataclass, field
from enum import Enum, auto
from functools import cache
from typing import Generic, Optional, TypeVar
from itertools import combinations

from single_types import ATTACK_TYPE_CHART, Type

class Effectiveness(Enum):
    NO_EFFECT = lambda x: x == 0.0
    QUARTER_EFFECTIVE = lambda x: x == 0.25
    HALF_EFFECTIVE = lambda x: x == 0.5
    NORMAL_EFFECTIVE = lambda x: x == 1.0
    DOUBLE_EFFECTIVE = lambda x: x == 2.0
    MORE_EFFECTIVE = lambda x: x > 1.0
    LESS_EFFECTIVE = lambda x: x < 1.0

T = TypeVar("T")

@dataclass(frozen=True)
class Relationship(Generic[T]):
    relationship: dict[T, float] = field(default_factory=dict)

    def effective_attacks(self, effectiveness: Effectiveness) -> set[T]:
        return {k for k, v in self.relationship.items() if effectiveness(v)}
    
    def items(self):
        return self.relationship.items()

@dataclass(frozen=True, init=False)
class MultiType:
    _types: frozenset[Type]

    def __repr__(self):
        return f"{self.__class__.__name__}({', '.join(map(repr, self._types))})"

    def __init__(self, *types: Type) -> MultiType:
        object.__setattr__(self, "_types", frozenset(types))

    @staticmethod
    def all_types(type_count: int = 1) -> set[MultiType]:
        return {MultiType(*types) for types in combinations(Type, type_count)}

    @property
    def defense(self) -> Relationship[Type]:
        """
        Defensive properties
        """
        type_multipliers = dict.fromkeys(Type, 1.0)
        for attack_type, relationship in ATTACK_TYPE_CHART.items():
            type_multipliers[attack_type] *= 0.0 if self._types & relationship.no_effect else 1.0
            type_multipliers[attack_type] *= 0.5 if self._types & relationship.half_effective else 1.0
            type_multipliers[attack_type] *= 2.0 if self._types & relationship.double_effective else 1.0
        return Relationship(relationship=type_multipliers)

    # def attack_coverage(self, types: set[MultiType]) -> dict[MultiType, float]:
    #     """
    #     Attack coverage
    #     """
    #     type_multipliers = dict.fromkeys(types, 1.0)
    #     for t in types:
    #         type_multipliers[t] = max(
    #             t.defense.relationship[attack_type]
    #             for attack_type in self._types
    #         )
    #     return type_multipliers
    
    def attack_coverage(self, types: set[MultiType]) -> Relationship[MultiType]:
        """
        Attack coverage
        """
        type_multipliers = {}
        for t in types:
            type_multipliers[t] = max(
                t.defense.relationship[attack_type]
                for attack_type in self._types
            )
        return Relationship(relationship=type_multipliers)
