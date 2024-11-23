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
class FloatRelationship:
    relationship: dict[Type, float] = field(default_factory=dict)

    def effective_attacks(self, effectiveness: Effectiveness) -> set[Type]:
        return {k for k, v in self.relationship.items() if effectiveness(v)}

@dataclass(frozen=True)
class MultiType:
    types: set[Type]

    @staticmethod
    def all_types(type_count: int) -> set[MultiType]:
        return {MultiType(set(types)) for types in combinations(Type, type_count)}

    def defense(self) -> FloatRelationship:
        type_multipliers = dict.fromkeys(Type, 1.0)
        for attack_type, relationship in ATTACK_TYPE_CHART.items():
            type_multipliers[attack_type] *= 0.0 if self.types & relationship.no_effect else 1.0
            type_multipliers[attack_type] *= 0.5 if self.types & relationship.half_effective else 1.0
            type_multipliers[attack_type] *= 2.0 if self.types & relationship.double_effective else 1.0
        return FloatRelationship(relationship=type_multipliers)
