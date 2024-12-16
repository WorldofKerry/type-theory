from __future__ import annotations
from collections import Counter
from dataclasses import dataclass, field
from enum import Enum, auto
from functools import cache
from typing import Callable, Generic, Optional, TypeVar
from itertools import combinations
from type_chart import ATTACK_TYPE_CHART, Type

class Effectiveness(Enum):
    NO_EFFECT = lambda x: x == 0.0
    QUARTER_EFFECTIVE = lambda x: x == 0.25
    HALF_EFFECTIVE = lambda x: x == 0.5
    NORMAL_EFFECTIVE = lambda x: x == 1.0
    DOUBLE_EFFECTIVE = lambda x: x == 2.0
    QUADRUPLE_EFFECTIVE = lambda x: x == 4.0

    MORE_EFFECTIVE = lambda x: x > 1.0
    LESS_EFFECTIVE = lambda x: x < 1.0

T = TypeVar("T")

class Relationship(dict[T, float]):
    def __init__(self, *args, **kwargs):
        self.update(*args, **kwargs)

    def filter(self, func: Callable[[float], bool] | Effectiveness) -> Relationship[T]:
        return Relationship({k: v for k, v in self.items() if func(v)})

@dataclass(frozen=True, init=False)
class MultiType:
    _types: frozenset[Type]

    def __repr__(self):
        return f"{self.__class__.__name__}({', '.join(map(repr, self._types))})"

    def __init__(self, *types: Type) -> MultiType:
        object.__setattr__(self, "_types", frozenset(types))

    @staticmethod
    def _all_types(type_count: int) -> set[MultiType]:
        return {MultiType(*types) for types in combinations(Type, type_count)}

    @staticmethod
    def all_types(type_count: int = 1, include_less: bool = True) -> set[MultiType]:
        if not include_less or type_count == 1:
            return MultiType._all_types(type_count)
        return MultiType._all_types(type_count) | MultiType.all_types(type_count - 1)

    @cache
    def defense(self) -> Relationship[Type]:
        """
        Defensive properties
        """
        type_multipliers = dict.fromkeys(Type, 1.0)
        for attack_type, relationship in ATTACK_TYPE_CHART.items():
            for t in self._types:
                type_multipliers[attack_type] *= 0.0 if t in relationship.no_effect else 1.0
                type_multipliers[attack_type] *= 0.5 if t in relationship.half_effective else 1.0
                type_multipliers[attack_type] *= 2.0 if t in relationship.double_effective else 1.0
        return Relationship(type_multipliers)

    def attack_coverage(self, types: set[MultiType]) -> Relationship[MultiType]:
        type_multipliers = {}
        for t in types:
            type_multipliers[t] = max(
                t.defense()[attack_type]
                for attack_type in self._types
            )
        return Relationship(type_multipliers)

    def resisted_by(self, types: set[MultiType]) -> Relationship[MultiType]:
        type_multipliers = {}
        for t in types:
            type_multipliers[t] = min(
                t.defense()[attack_type]
                for attack_type in self._types
            )
        return Relationship(type_multipliers)

@dataclass(frozen=True, init=False)
class Team:
    _members: frozenset[MultiType]

    def __repr__(self):
        return f"{self.__class__.__name__}({', '.join(map(repr, self._members))})"
    
    def __init__(self, *members: MultiType) -> Team:
        object.__setattr__(self, "_members", frozenset(members))

    def __hash__(self):
        return hash(self._members)
    
    def __eq__(self, value):
        return isinstance(value, Team) and self._members == value._members

    def weaknesses_count(self) -> dict[Type, int]:
        return Type.natural_order(Counter(
            attack_type
            for member in self._members
            for attack_type, multiplier in member.defense().items()
            if multiplier > 1.0
        ))
    
    def resistances_count(self) -> dict[Type, int]:
        return Type.natural_order(Counter(
            attack_type
            for member in self._members
            for attack_type, multiplier in member.defense().items()
            if multiplier < 1.0
        ))
