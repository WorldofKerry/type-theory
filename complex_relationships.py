from __future__ import annotations
from dataclasses import dataclass, field
from enum import Enum, auto
from functools import cache
from typing import Optional
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

@dataclass(frozen=True)
class Relationship:
    relationship: dict[Type, float] = field(default_factory=dict)

    def effective_attacks(self, effectiveness: Effectiveness) -> set[Type]:
        return {k for k, v in self.relationship.items() if effectiveness(v)}

@dataclass(frozen=True)
class MultiType:
    _types: frozenset[Type]

    @classmethod
    def from_types(cls, *types: Type) -> MultiType:
        return cls(frozenset(types))

    @staticmethod
    def all_types(type_count: int = 1) -> set[MultiType]:
        combs = list(combinations(Type, type_count))
        ret = set()
        for types in combs:
            ret.add(MultiType(frozenset(types)))
        return ret

    @property
    def defense(self) -> Relationship:
        type_multipliers = dict.fromkeys(Type, 1.0)
        for attack_type, relationship in ATTACK_TYPE_CHART.items():
            type_multipliers[attack_type] *= 0.0 if self._types & relationship.no_effect else 1.0
            type_multipliers[attack_type] *= 0.5 if self._types & relationship.half_effective else 1.0
            type_multipliers[attack_type] *= 2.0 if self._types & relationship.double_effective else 1.0
        return Relationship(relationship=type_multipliers)
