use crate::types::*;

pub const TWO_FIVE_ONE: [GenericChordSynced; 3] = [
    GenericChordSynced {generic_chord: GenericChord{scale_tone: ScaleTone::Two, chord_type: ChordType::MinorSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord{scale_tone: ScaleTone::Five, chord_type: ChordType::DominantSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord{scale_tone: ScaleTone::One, chord_type: ChordType::MajorSeven}, beats: 4},
];

pub const JAPAN_GAMERS: [GenericChordSynced; 4] = [
    GenericChordSynced {generic_chord: GenericChord{scale_tone: ScaleTone::Four, chord_type: ChordType::MajorSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord{scale_tone: ScaleTone::Five, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord{scale_tone: ScaleTone::Three, chord_type: ChordType::MinorSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord{scale_tone: ScaleTone::Six, chord_type: ChordType::MinorSeven}, beats: 4},
];

pub const BLUES_STANDARD: [GenericChordSynced; 12] = [
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Four, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Four, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Five, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Four, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven}, beats: 4},
];

pub const BLUES_QUICK_CHANGE: [GenericChordSynced; 12] = [
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Four, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Four, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Four, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Five, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Four, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Five, chord_type: ChordType::DominantSeven}, beats: 4},
];
pub const BLUES_BASIE: [GenericChordSynced; 14] = [
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Four, chord_type: ChordType::DominantSevenFlatNine}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::FlatFive, chord_type: ChordType::Diminished}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Five, chord_type: ChordType::MinorSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Four, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::FlatFive, chord_type: ChordType::Diminished}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Six, chord_type: ChordType::DominantSevenFlatNine}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Two, chord_type: ChordType::MinorSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Five, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven}, beats: 4},
];
pub const BLUES_BEBOP: [GenericChordSynced; 16] = [
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Four, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Five, chord_type: ChordType::MinorSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Four, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::FlatFive, chord_type: ChordType::Diminished}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Three, chord_type: ChordType::MinorSevenFlatFive}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Six, chord_type: ChordType::DominantSevenFlatNine}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Two, chord_type: ChordType::MinorSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Five, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Three, chord_type: ChordType::MinorSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Six, chord_type: ChordType::DominantSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Two, chord_type: ChordType::MinorSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Five, chord_type: ChordType::DominantSeven}, beats: 2},
];
pub const BLUES_TRITONE_SUB: [GenericChordSynced; 19] = [
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Four, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Five, chord_type: ChordType::MinorSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Four, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::FlatFive, chord_type: ChordType::MinorSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Seven, chord_type: ChordType::DominantSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Seven, chord_type: ChordType::DominantSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::FlatSeven, chord_type: ChordType::DominantSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Six, chord_type: ChordType::DominantSevenFlatNine}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Two, chord_type: ChordType::MinorSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Five, chord_type: ChordType::DominantSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Four, chord_type: ChordType::DominantSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Three, chord_type: ChordType::MinorSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Six, chord_type: ChordType::DominantSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Two, chord_type: ChordType::MinorSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Five, chord_type: ChordType::DominantSeven}, beats: 2},
];
pub const BLUES_BIRD: [GenericChordSynced; 20] = [
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::MajorSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Seven, chord_type: ChordType::MinorSevenFlatFive}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Three, chord_type: ChordType::DominantSevenFlatNine}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Six, chord_type: ChordType::MinorSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Two, chord_type: ChordType::DominantSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Five, chord_type: ChordType::MinorSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::One, chord_type: ChordType::DominantSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Four, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Four, chord_type: ChordType::MinorSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::FlatSeven, chord_type: ChordType::DominantSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Three, chord_type: ChordType::MinorSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Six, chord_type: ChordType::DominantSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::FlatThree, chord_type: ChordType::MinorSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::FlatSix, chord_type: ChordType::DominantSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Two, chord_type: ChordType::MinorSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Five, chord_type: ChordType::DominantSeven}, beats: 4},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Three, chord_type: ChordType::Major}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Six, chord_type: ChordType::DominantSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Two, chord_type: ChordType::MinorSeven}, beats: 2},
    GenericChordSynced {generic_chord: GenericChord {scale_tone: ScaleTone::Five, chord_type: ChordType::DominantSeven}, beats: 2},
];
