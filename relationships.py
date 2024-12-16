from __future__ import annotations
from collections import Counter, defaultdict
from enum import Enum, auto
from functools import cache
from statistics import geometric_mean
from typing import Callable, Generic, Optional, Sequence, TypeVar
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

class MultiType:
    _types: frozenset[Type]

    def __repr__(self):
        return f"{self.__class__.__name__}({', '.join(map(repr, Type.natural_order(self._types)))})"

    def __init__(self, *types: Type) -> MultiType:
        self._types = frozenset(types)

    def __hash__(self):
        return hash(self._types)
    
    def __eq__(self, value):
        return isinstance(value, MultiType) and self._types == value._types

    @staticmethod
    def _all_types(type_count: int, include_abilities: bool) -> set[MultiType]:
        if include_abilities:
            return {MultiType(*types) for types in combinations(Type, type_count)}
        return {MultiType(*types) for types in combinations(Type.basic(), type_count)}

    @staticmethod
    def all_types(type_count: int = 1, include_less: bool = True, include_abilities: bool = False) -> set[MultiType]:
        if not include_less or type_count == 1:
            return MultiType._all_types(type_count, include_abilities)
        return MultiType._all_types(type_count, include_abilities) | MultiType.all_types(type_count - 1, include_abilities)

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

class Team:
    _members: frozenset[MultiType]

    @classmethod
    def from_list(cls, types: Sequence[MultiType]) -> Team:
        return cls(*types)

    def __repr__(self):
        return f"{self.__class__.__name__}({', '.join(map(repr, self._members))})"
    
    def __init__(self, *members: MultiType) -> Team:
        self._members = frozenset(members)

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

    def average_damage(self, immunity_multiplier: float = 0.0) -> float:
        """
        If every team member took 1 damage from every type of attack, what is the average damage taken?
        """
        total_damage = 0
        for member in self._members:
            for _attack_type, multiplier in member.defense().items():
                if multiplier == 0.0:
                    multiplier = immunity_multiplier
                total_damage += 1 * multiplier
        return total_damage / len(self._members)
