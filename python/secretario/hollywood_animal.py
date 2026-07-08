"""
Hollywood Animal Catalog - Compatibility Matrix Implementation

This module provides the Hollywood Animal catalog with compatibility scoring
for narrative elements (protagonists, antagonists, themes, events, etc.).
"""

import csv
import os
from typing import Dict, List, Optional, Any, Set
from dataclasses import dataclass, field, asdict
from pathlib import Path


@dataclass
class ToneProfile:
    serious: float = 0.0
    comic: float = 0.0
    grim: float = 0.0
    adventurous: float = 0.0
    melodramatic: float = 0.0
    pulpy: float = 0.0


@dataclass
class MoralProfile:
    idealist: float = 0.0
    cynical: float = 0.0
    corrupt: float = 0.0
    redemptive: float = 0.0
    lawful: float = 0.0
    chaotic: float = 0.0


@dataclass
class RealismProfile:
    grounded: float = 0.0
    heightened: float = 0.0
    fantastical: float = 0.0
    supernatural: float = 0.0
    sci_fi: float = 0.0


@dataclass
class HollywoodElement:
    id: str
    category: str
    subtype: str
    narrative_scale: str
    tone_profile: ToneProfile
    moral_profile: MoralProfile
    realism_profile: RealismProfile
    agency_type: str
    core_drives: List[str] = field(default_factory=list)
    content_flags: List[str] = field(default_factory=list)
    genre_affinity: List[str] = field(default_factory=list)
    setting_affinity: List[str] = field(default_factory=list)


@dataclass
class CompatibilityResult:
    score: float
    category: str
    axis_scores: Dict[str, float]
    penalties: List[str]
    bonuses: List[str]
    explanation: List[str]


SCALE_VALUES = {
    "intimate": 0.0,
    "personal": 0.25,
    "local": 0.5,
    "societal": 0.75,
    "epic": 1.0,
}


class HollywoodAnimalCatalog:
    def __init__(self, data_dir: str = "data/hollywood_animal"):
        self.data_dir = Path(data_dir)
        self.elements: Dict[str, HollywoodElement] = {}
        self.rules: Dict[str, Dict[str, float]] = {}
        self._load_elements()
        self._load_rules()

    def _load_elements(self) -> None:
        elements_path = self.data_dir / "elements.csv"
        if not elements_path.exists():
            raise FileNotFoundError(f"elements.csv not found at {elements_path}")
        with open(elements_path, 'r', encoding='utf-8') as f:
            reader = csv.DictReader(f)
            for row in reader:
                elem = HollywoodElement(
                    id=row['id'],
                    category=row['category'],
                    subtype=row['subtype'],
                    narrative_scale=row['narrative_scale'],
                    tone_profile=ToneProfile(
                        serious=float(row.get('tone_serious', 0)),
                        comic=float(row.get('tone_comic', 0)),
                        grim=float(row.get('tone_grim', 0)),
                        adventurous=float(row.get('tone_adventurous', 0)),
                        melodramatic=float(row.get('tone_melodramatic', 0)),
                        pulpy=float(row.get('tone_pulpy', 0))
                    ),
                    moral_profile=MoralProfile(
                        idealist=float(row.get('moral_idealist', 0)),
                        cynical=float(row.get('moral_cynical', 0)),
                        corrupt=float(row.get('moral_corrupt', 0)),
                        redemptive=float(row.get('moral_redemptive', 0)),
                        lawful=float(row.get('moral_lawful', 0)),
                        chaotic=float(row.get('moral_chaotic', 0))
                    ),
                    realism_profile=RealismProfile(
                        grounded=float(row.get('realism_grounded', 0)),
                        heightened=float(row.get('realism_heightened', 0)),
                        fantastical=float(row.get('realism_fantastical', 0)),
                        supernatural=float(row.get('realism_supernatural', 0)),
                        sci_fi=float(row.get('realism_sci_fi', 0))
                    ),
                    agency_type=row.get('agency_type', ''),
                    core_drives=row.get('core_drives', '').split(';'),
                    content_flags=[f for f in row.get('content_flags', '').split(';') if f],
                    genre_affinity=row.get('genre_affinity', '').split(';'),
                    setting_affinity=row.get('setting_affinity', '').split(';')
                )
                self.elements[elem.id] = elem

    def _load_rules(self) -> None:
        rules_path = self.data_dir / "compatibility_rules.csv"
        if not rules_path.exists():
            raise FileNotFoundError(f"compatibility_rules.csv not found at {rules_path}")
        with open(rules_path, 'r', encoding='utf-8') as f:
            reader = csv.DictReader(f)
            for row in reader:
                pair_type = row['pair_type']
                axis = row['axis']
                weight = float(row['weight'])
                if pair_type not in self.rules:
                    self.rules[pair_type] = {}
                self.rules[pair_type][axis] = weight

    def get_pair_type(self, a: HollywoodElement, b: HollywoodElement) -> str:
        cats = sorted([a.category, b.category])
        return f"{cats[0]}_{cats[1]}"

    def calculate_axis_score(self, axis: str, a: HollywoodElement, b: HollywoodElement) -> float:
        if axis == "role_complementarity": return self._role_complementarity(a, b)
        elif axis == "moral_tension": return self._moral_tension(a, b)
        elif axis == "tone_alignment": return self._tone_alignment(a, b)
        elif axis == "scale_alignment": return self._scale_alignment(a, b)
        elif axis == "genre_support": return self._genre_support(a, b)
        elif axis == "setting_fit": return self._setting_fit(a, b)
        elif axis == "arc_coherence": return self._arc_coherence(a, b)
        elif axis == "embodiment_strength": return self._embodiment_strength(a, b)
        elif axis == "arc_payoff": return self._arc_payoff(a, b)
        elif axis == "core_drives_overlap": return self._core_drives_overlap(a, b)
        elif axis == "moral_alignment": return self._moral_alignment(a, b)
        elif axis == "moral_coherence": return self._moral_coherence(a, b)
        elif axis == "natural_fit": return self._natural_fit(a, b)
        elif axis == "reinterpretation_fit": return self._reinterpretation_fit(a, b)
        elif axis == "novelty_bonus": return self._novelty_bonus(a, b)
        elif axis == "sensitivity_risk": return self._sensitivity_risk(a, b)
        else: return 0.5

    def calculate_compatibility(self, a_id: str, b_id: str, preset: Optional[str] = None) -> CompatibilityResult:
        if a_id not in self.elements:
            raise ValueError(f"Element {a_id} not found")
        if b_id not in self.elements:
            raise ValueError(f"Element {b_id} not found")
        a = self.elements[a_id]
        b = self.elements[b_id]
        pair_type = self.get_pair_type(a, b)
        rules = self.rules.get(pair_type, self.rules.get('any_any', {}))
        axis_scores = {}
        total = 0.0
        for axis, weight in rules.items():
            score = self.calculate_axis_score(axis, a, b)
            axis_scores[axis] = score
            total += weight * score
        penalties = self._calculate_penalties(a, b)
        bonuses = self._calculate_bonuses(a, b)
        final_score = max(0.0, min(1.0, total - penalties + bonuses))
        category = self._discretize_score(final_score)
        explanation = self._generate_explanation(a, b, axis_scores, penalties, bonuses)
        return CompatibilityResult(
            score=final_score, category=category, axis_scores=axis_scores,
            penalties=[f"Total penalty: {penalties:.4f}"], bonuses=[f"Total bonus: {bonuses:.4f}"],
            explanation=explanation
        )

    def calculate_set_compatibility(self, element_ids: List[str], preset: Optional[str] = None) -> Dict[str, Any]:
        scores = {}
        for i in range(len(element_ids)):
            for j in range(i + 1, len(element_ids)):
                a_id = element_ids[i]
                b_id = element_ids[j]
                try:
                    result = self.calculate_compatibility(a_id, b_id, preset)
                    scores[f"{a_id}_{b_id}"] = result.score
                except ValueError as e:
                    scores[f"{a_id}_{b_id}_error"] = str(e)
        if scores:
            scores["internal_coherence"] = sum(scores.values()) / len(scores)
        return scores

    def get_element(self, element_id: str) -> Optional[HollywoodElement]:
        return self.elements.get(element_id)

    def get_all_elements(self) -> List[HollywoodElement]:
        return list(self.elements.values())

    def get_elements_by_category(self, category: str) -> List[HollywoodElement]:
        return [e for e in self.elements.values() if e.category == category]

    def get_categories(self) -> Set[str]:
        return {e.category for e in self.elements.values()}

    def search_elements(self, query: str) -> List[HollywoodElement]:
        query_lower = query.lower()
        return [e for e in self.elements.values() if query_lower in e.id.lower()]

    def _role_complementarity(self, a: HollywoodElement, b: HollywoodElement) -> float:
        if (a.category == 'protagonist' and b.category == 'antagonist') or (a.category == 'antagonist' and b.category == 'protagonist'): return 0.95
        if (a.id == 'MENTOR' and b.id == 'ACCIDENTAL_HERO') or (a.id == 'ACCIDENTAL_HERO' and b.id == 'MENTOR'): return 0.98
        if (a.id == 'KNIGHT' and b.id == 'CORRUPT_OFFICIAL') or (a.id == 'CORRUPT_OFFICIAL' and b.id == 'KNIGHT'): return 0.95
        if (a.id == 'DETECTIVE' and b.id == 'CRIMINAL_MASTERMIND') or (a.id == 'CRIMINAL_MASTERMIND' and b.id == 'DETECTIVE'): return 0.95
        if a.category == b.category: return 0.4
        if (a.category == 'supporting' and b.category == 'protagonist') or (a.category == 'protagonist' and b.category == 'supporting'): return 0.7
        if (a.category == 'supporting' and b.category == 'antagonist') or (a.category == 'antagonist' and b.category == 'supporting'): return 0.5
        return 0.5

    def _moral_tension(self, a: HollywoodElement, b: HollywoodElement) -> float:
        ma, mb = a.moral_profile, b.moral_profile
        return min(((abs(ma.idealist - mb.corrupt) + abs(ma.corrupt - mb.idealist) + abs(ma.lawful - mb.chaotic) + abs(ma.chaotic - mb.lawful)) / 4.0), 1.0)

    def _tone_alignment(self, a: HollywoodElement, b: HollywoodElement) -> float:
        ta, tb = a.tone_profile, b.tone_profile
        diff = (abs(ta.serious - tb.serious) + abs(ta.comic - tb.comic) + abs(ta.grim - tb.grim) + abs(ta.adventurous - tb.adventurous) + abs(ta.melodramatic - tb.melodramatic) + abs(ta.pulpy - tb.pulpy)) / 6.0
        return 1.0 - diff

    def _scale_alignment(self, a: HollywoodElement, b: HollywoodElement) -> float:
        sa = SCALE_VALUES.get(a.narrative_scale, 0.5)
        sb = SCALE_VALUES.get(b.narrative_scale, 0.5)
        return 1.0 - abs(sa - sb)

    def _genre_support(self, a: HollywoodElement, b: HollywoodElement) -> float:
        ag, bg = set(a.genre_affinity), set(b.genre_affinity)
        return min(len(ag & bg) / max(len(ag | bg), 1), 1.0)

    def _setting_fit(self, a: HollywoodElement, b: HollywoodElement) -> float:
        aset, bset = set(a.setting_affinity), set(b.setting_affinity)
        return min(len(aset & bset) / max(len(aset | bset), 1), 1.0)

    def _arc_coherence(self, a: HollywoodElement, b: HollywoodElement) -> float:
        if (a.category == 'protagonist' and b.category == 'theme') or (a.category == 'theme' and b.category == 'protagonist'):
            p = a if a.category == 'protagonist' else b
            t = b if a.category == 'protagonist' else a
            pd = set(p.core_drives)
            if 'love' in t.id.lower() and 'love' in pd: return 0.95
            if 'justice' in t.id.lower() and 'justice' in pd: return 0.95
            if 'freedom' in t.id.lower() and 'freedom' in pd: return 0.95
            if 'war' in t.id.lower() and 'survival' in pd: return 0.90
            if 'treasure' in t.id.lower() and 'greed' in pd: return 0.85
            return 0.6
        if (a.category == 'theme' and b.category == 'event') or (a.category == 'event' and b.category == 'theme'):
            return self._embodiment_strength(a, b)
        return 0.5

    def _embodiment_strength(self, a: HollywoodElement, b: HollywoodElement) -> float:
        if (a.category == 'theme' and b.category == 'event') or (a.category == 'event' and b.category == 'theme'):
            theme = a if a.category == 'theme' else b
            event = b if a.category == 'theme' else a
            if theme.id == 'THEME_PROTECTING_THE_WITNESS' and event.id == 'EVENTS_AMBUSH': return 0.95
            if theme.id == 'THEME_LONG_JOURNEY' and event.id == 'EVENTS_WORLDWIDE_TRAVELING': return 0.95
            if theme.id == 'THEME_FIGHT_FOR_FREEDOM' and event.id == 'EVENTS_ESCAPE_CAPTIVITY': return 0.95
            if theme.id == 'THEME_AVENGING_LOVED_ONES' and event.id == 'EVENTS_FINAL_SHOWDOWN': return 0.90
            if theme.id == 'THEME_TREASURE_HUNT' and event.id == 'EVENTS_ANCIENT_PUZZLE': return 0.90
            if theme.id == 'THEME_SURVIVING_IN_WARTIME' and event.id == 'EVENTS_BIG_BATTLE_SCENES': return 0.90
            if 'war' in theme.id.lower() and 'battle' in event.id.lower(): return 0.85
            if 'love' in theme.id.lower() and 'romantic' in event.id.lower(): return 0.80
            return 0.7
        return 0.5

    def _arc_payoff(self, a: HollywoodElement, b: HollywoodElement) -> float:
        if (a.category == 'theme' and b.category == 'finale') or (a.category == 'finale' and b.category == 'theme'):
            theme = a if a.category == 'theme' else b
            finale = b if a.category == 'theme' else a
            if theme.id == 'THEME_UNREQUITED_LOVE' and finale.id == 'STARCROSSED_LOVERS': return 0.95
            if theme.id == 'THEME_STRUGGLE_FOR_BETTER_LIFE' and finale.id == 'PROTAGONISTS_DREAMS_CRUSHED': return 0.90
            if theme.id == 'THEME_AVENGING_LOVED_ONES' and finale.id == 'ANTAGONIST_GETS_KILLED': return 0.95
            if theme.id == 'THEME_FIGHT_FOR_FREEDOM' and finale.id == 'PROTAGONIST_FINDS_TREASURE': return 0.10
            if theme.id == 'THEME_FIGHT_FOR_FREEDOM' and finale.id == 'PROTAGONIST_FINDS_LOVE': return 0.85
            if theme.id == 'THEME_LONG_JOURNEY' and finale.id == 'PROTAGONIST_RETURNS_HOME': return 0.95
            if theme.id == 'THEME_WAR_IS_HELL' and finale.id == 'PROTAGONIST_DIES_HEROICALLY': return 0.95
            if 'love' in theme.id.lower() and 'love' in finale.id.lower(): return 0.85
            if 'justice' in theme.id.lower() and 'justice' in finale.id.lower(): return 0.85
            return 0.7
        return 0.5

    def _core_drives_overlap(self, a: HollywoodElement, b: HollywoodElement) -> float:
        ad, bd = set(a.core_drives), set(b.core_drives)
        return min(len(ad & bd) / max(len(ad | bd), 1), 1.0)

    def _moral_alignment(self, a: HollywoodElement, b: HollywoodElement) -> float:
        ma, mb = a.moral_profile, b.moral_profile
        diff = (abs(ma.idealist - mb.idealist) + abs(ma.cynical - mb.cynical) + abs(ma.corrupt - mb.corrupt) + abs(ma.redemptive - mb.redemptive) + abs(ma.lawful - mb.lawful) + abs(ma.chaotic - mb.chaotic)) / 6.0
        return 1.0 - diff

    def _moral_coherence(self, a: HollywoodElement, b: HollywoodElement) -> float:
        return self._moral_alignment(a, b)

    def _natural_fit(self, a: HollywoodElement, b: HollywoodElement) -> float:
        return self._setting_fit(a, b)

    def _reinterpretation_fit(self, a: HollywoodElement, b: HollywoodElement) -> float:
        if (a.id == 'KNIGHT' and any('MODERN' in s for s in b.setting_affinity)) or (b.id == 'KNIGHT' and any('MODERN' in s for s in a.setting_affinity)): return 0.85
        if (a.id == 'ROBOT' and any('MEDIEVAL' in s for s in b.setting_affinity)) or (b.id == 'ROBOT' and any('MEDIEVAL' in s for s in a.setting_affinity)): return 0.30
        if (a.id == 'COWBOY' and any('SPACE' in s for s in b.setting_affinity)) or (b.id == 'COWBOY' and any('SPACE' in s for s in a.setting_affinity)): return 0.60
        return 0.5

    def _novelty_bonus(self, a: HollywoodElement, b: HollywoodElement) -> float:
        if (a.id == 'ACCIDENTAL_HERO' and b.id == 'ANCIENT_EVIL') or (a.id == 'ANCIENT_EVIL' and b.id == 'ACCIDENTAL_HERO'): return 0.10
        if (a.id == 'ROBOT' and any('DYSTOPIAN' in s for s in b.setting_affinity)) or (b.id == 'ROBOT' and any('DYSTOPIAN' in s for s in a.setting_affinity)): return 0.15
        if (a.id == 'CLUMSY_OAF' and b.tone_profile.serious > 0.7) or (b.id == 'CLUMSY_OAF' and a.tone_profile.serious > 0.7): return 0.05
        return 0.0

    def _sensitivity_risk(self, a: HollywoodElement, b: HollywoodElement) -> float:
        if (a.tone_profile.comic > 0.5 and 'extreme_violence' in b.content_flags) or (b.tone_profile.comic > 0.5 and 'extreme_violence' in a.content_flags): return 1.0
        if (a.tone_profile.serious > 0.7 and 'comedy' in b.genre_affinity) or (b.tone_profile.serious > 0.7 and 'comedy' in a.genre_affinity): return 0.8
        if a.moral_profile.idealist > 0.7 and b.moral_profile.corrupt > 0.7: return 0.7
        if (a.tone_profile.grim > 0.7 and b.tone_profile.comic > 0.5) or (b.tone_profile.grim > 0.7 and a.tone_profile.comic > 0.5): return 0.6
        if (a.tone_profile.melodramatic > 0.7 and b.tone_profile.pulpy > 0.7) or (b.tone_profile.melodramatic > 0.7 and a.tone_profile.pulpy > 0.7): return 0.3
        return 0.0

    def _calculate_penalties(self, a: HollywoodElement, b: HollywoodElement) -> float:
        sensitivity = self._sensitivity_risk(a, b)
        if 'any_any' in self.rules and 'sensitivity_risk' in self.rules['any_any']:
            return sensitivity * self.rules['any_any']['sensitivity_risk']
        return sensitivity * 0.12

    def _calculate_bonuses(self, a: HollywoodElement, b: HollywoodElement) -> float:
        novelty = self._novelty_bonus(a, b)
        if 'any_any' in self.rules and 'novelty_bonus' in self.rules['any_any']:
            return novelty * self.rules['any_any']['novelty_bonus']
        return novelty * 0.04

    def _discretize_score(self, score: float) -> str:
        if score >= 0.80: return "strong"
        elif score >= 0.60: return "good"
        elif score >= 0.40: return "conditional"
        elif score >= 0.20: return "weak"
        else: return "incompatible"

    def _generate_explanation(self, a: HollywoodElement, b: HollywoodElement, axis_scores: Dict[str, float], penalties: float, bonuses: float) -> List[str]:
        explanations = []
        for axis, score in axis_scores.items():
            if score >= 0.8:
                if axis == "role_complementarity":
                    explanations.append(f"ROLE_COMPLEMENTARITY: {a.id} and {b.id} have complementary narrative roles")
                elif axis == "moral_tension":
                    explanations.append(f"MORAL_TENSION: Strong ethical contrast between {a.id} and {b.id}")
                elif axis == "tone_alignment":
                    explanations.append(f"TONE_ALIGNMENT: {a.id} and {b.id} share similar tonal profiles")
                elif axis == "embodiment_strength":
                    eid = b.id if b.category == 'event' else a.id
                    tid = a.id if a.category == 'theme' else b.id
                    explanations.append(f"EMBODIMENT: {eid} strongly embodies the theme of {tid}")
                elif axis == "arc_payoff":
                    fid = b.id if b.category == 'finale' else a.id
                    tid = a.id if a.category == 'theme' else b.id
                    explanations.append(f"ARC_PAYOFF: {fid} provides strong payoff for theme {tid}")
        if penalties > 0.0:
            if (a.tone_profile.comic > 0.5 and 'extreme_violence' in b.content_flags) or (b.tone_profile.comic > 0.5 and 'extreme_violence' in a.content_flags):
                explanations.append("PENALTY: Comic tone clashes with extreme violence content")
            if (a.tone_profile.serious > 0.7 and 'comedy' in b.genre_affinity) or (b.tone_profile.serious > 0.7 and 'comedy' in a.genre_affinity):
                explanations.append("PENALTY: Serious tone clashes with comedy genre")
            if a.moral_profile.idealist > 0.7 and b.moral_profile.corrupt > 0.7:
                explanations.append("PENALTY: Idealist and corrupt profiles create moral dissonance")
        if bonuses > 0.0:
            if (a.id == 'ACCIDENTAL_HERO' and b.id == 'ANCIENT_EVIL') or (a.id == 'ANCIENT_EVIL' and b.id == 'ACCIDENTAL_HERO'):
                explanations.append("BONUS: Rare but valid combination of accidental hero vs ancient evil")
            if (a.id == 'ROBOT' and any('DYSTOPIAN' in s for s in b.setting_affinity)) or (b.id == 'ROBOT' and any('DYSTOPIAN' in s for s in a.setting_affinity)):
                explanations.append("BONUS: Robot in dystopian setting receives contextual boost")
        return explanations


def get_catalog(data_dir: str = "data/hollywood_animal") -> HollywoodAnimalCatalog:
    return HollywoodAnimalCatalog(data_dir)
