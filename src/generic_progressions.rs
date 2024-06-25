use crate::*;

pub const TWO_FIVE_ONE: [GenericChord; 3] = [
    GenericChord {scale_tone: ScaleTone::Two, chord_type: ChordType::MinorSeven},
    GenericChord {scale_tone: ScaleTone::Five, chord_type: ChordType::DominantSeven},
    GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::MajorSeven},
];

pub const BLUES_STANDARD: [GenericChord; 12] = [
    GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven},
    GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven},
    GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven},
    GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven},
    GenericChord {scale_tone: ScaleTone::Four, chord_type: ChordType::DominantSeven},
    GenericChord {scale_tone: ScaleTone::Four, chord_type: ChordType::DominantSeven},
    GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven},
    GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven},
    GenericChord {scale_tone: ScaleTone::Five, chord_type: ChordType::DominantSeven},
    GenericChord {scale_tone: ScaleTone::Four, chord_type: ChordType::DominantSeven},
    GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven},
    GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven},
];

pub const BLUES_QUICK_CHANGE: [GenericChord; 12] = [
    GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven},
    GenericChord {scale_tone: ScaleTone::Four, chord_type: ChordType::DominantSeven},
    GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven},
    GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven},
    GenericChord {scale_tone: ScaleTone::Four, chord_type: ChordType::DominantSeven},
    GenericChord {scale_tone: ScaleTone::Four, chord_type: ChordType::DominantSeven},
    GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven},
    GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven},
    GenericChord {scale_tone: ScaleTone::Five, chord_type: ChordType::DominantSeven},
    GenericChord {scale_tone: ScaleTone::Four, chord_type: ChordType::DominantSeven},
    GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven},
    GenericChord {scale_tone: ScaleTone::Five, chord_type: ChordType::DominantSeven},
];
