use std::fmt;
use std::fmt::Formatter;

use rand::{random, Rng};
use rand::distributions::{Distribution, Standard};
use strum_macros::EnumString;

use crate::helpers::*;

pub static NOTE_MAP: [Note; 12] = [
    Note { raw_note: RawNote('A'), pitch: Pitch::Natural },
    Note { raw_note: RawNote('A'), pitch: Pitch::Sharp },
    Note { raw_note: RawNote('B'), pitch: Pitch::Natural },
    Note { raw_note: RawNote('C'), pitch: Pitch::Natural },
    Note { raw_note: RawNote('C'), pitch: Pitch::Sharp },
    Note { raw_note: RawNote('D'), pitch: Pitch::Natural },
    Note { raw_note: RawNote('D'), pitch: Pitch::Sharp },
    Note { raw_note: RawNote('E'), pitch: Pitch::Natural },
    Note { raw_note: RawNote('F'), pitch: Pitch::Natural },
    Note { raw_note: RawNote('F'), pitch: Pitch::Sharp },
    Note { raw_note: RawNote('G'), pitch: Pitch::Natural },
    Note { raw_note: RawNote('G'), pitch: Pitch::Sharp },
];

pub const RAW_NOTE_MAP: [char; 7] = ['A', 'B', 'C', 'D', 'E', 'F', 'G'];

pub const TONE_MAP: [ScaleTone; 12] = [
    ScaleTone::One,
    ScaleTone::FlatTwo,
    ScaleTone::Two,
    ScaleTone::FlatThree,
    ScaleTone::Three,
    ScaleTone::Four,
    ScaleTone::FlatFive,
    ScaleTone::Five,
    ScaleTone::FlatSix,
    ScaleTone::Six,
    ScaleTone::FlatSeven,
    ScaleTone::Seven,
];

#[derive(Debug, PartialEq, Clone, EnumString)]
pub enum ScaleTone {
    #[strum(serialize = "I")]
    One,
    #[strum(serialize = "bII", serialize = "flatII")]
    FlatTwo,
    #[strum(serialize = "II")]
    Two,
    #[strum(serialize = "bIII", serialize = "flatIII")]
    FlatThree,
    #[strum(serialize = "III")]
    Three,
    #[strum(serialize = "IV")]
    Four,
    #[strum(serialize = "bV", serialize = "flatV")]
    FlatFive,
    #[strum(serialize = "V")]
    Five,
    #[strum(serialize = "bVI", serialize = "flatVI")]
    FlatSix,
    #[strum(serialize = "VI")]
    Six,
    #[strum(serialize = "bVII", serialize = "flatVII")]
    FlatSeven,
    #[strum(serialize = "VII")]
    Seven,
}
#[derive(Debug, PartialEq, Clone)]
pub struct GenericChord {
    pub scale_tone: ScaleTone,
    pub chord_type: ChordType,
}

impl GenericChord {
    pub fn new(scale_tone: ScaleTone, chord_type: ChordType) -> Self {
        GenericChord { scale_tone, chord_type }
    }

    pub fn into_chord(self, root: Note) -> Chord {
        let note = solve_interval(root.clone(), self.scale_tone);
        Chord::new(note, self.chord_type)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct GenericChordSynced {
    pub generic_chord: GenericChord,
    pub beats: u8,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChordSynced {
    pub chord: Chord,
    pub beats: u8,
}

pub struct ChordProgression {
    pub root: Note,
    pub chords: Vec<ChordSynced>,
    pub key: Key,
}

impl ChordProgression {
    pub fn new<I>(root: &Note, key: &Key, tones: &I) -> Self
    where
        I: IntoIterator<Item=GenericChordSynced> + Clone,
    {
        let chords = tones
            .clone()
            .into_iter()
            .map(|gc| ChordSynced {
                chord: gc.clone().generic_chord.into_chord(root.clone()),
                beats: gc.beats,
            })
            .collect::<Vec<ChordSynced>>();
        let root = root.to_owned();
        let key = key.to_owned();

        let mut res = ChordProgression {
            root,
            chords,
            key,
        };

        res.auto_convert_flats();
        res
    }

    pub fn auto_convert_flats(&mut self) {
        if self.key == Key::Major
            && ((self.root.raw_note.0 == 'F' && self.root.pitch == Pitch::Natural)
            || (self.root.raw_note.0 == 'A' && self.root.pitch == Pitch::Sharp)
            || (self.root.raw_note.0 == 'D' && self.root.pitch == Pitch::Sharp)
            || (self.root.raw_note.0 == 'G' && self.root.pitch == Pitch::Sharp)
            || (self.root.raw_note.0 == 'C' && self.root.pitch == Pitch::Sharp)) {
            for c in &mut self.chords {
                c.chord.note = c.chord.note.sharp_to_flat();
            }
        } else if self.key == Key::Minor
            && ((self.root.raw_note.0 == 'D' && self.root.pitch == Pitch::Natural)
            || (self.root.raw_note.0 == 'G' && self.root.pitch == Pitch::Natural)
            || (self.root.raw_note.0 == 'D' && self.root.pitch == Pitch::Natural)
            || (self.root.raw_note.0 == 'F' && self.root.pitch == Pitch::Natural)
            || (self.root.raw_note.0 == 'A' && self.root.pitch == Pitch::Sharp)) {
            for c in &mut self.chords {
                c.chord.note = c.chord.note.sharp_to_flat();
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Key {
    Major,
    Minor,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct RawNote(char);

impl RawNote {
    pub fn from(note: char) -> Self {
        if note < 'A' || note > 'G' {
            return RawNote('C');
        }
        RawNote(note)
    }
}

impl fmt::Display for RawNote {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Distribution<RawNote> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RawNote {
        match rng.gen_range(0..=6) {
            0 => RawNote::from('A'),
            1 => RawNote::from('B'),
            2 => RawNote::from('C'),
            3 => RawNote::from('D'),
            4 => RawNote::from('E'),
            5 => RawNote::from('F'),
            6 => RawNote::from('G'),
            _ => panic!("outside of note range")
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    pub raw_note: RawNote,
    pub pitch: Pitch,
}

impl Distribution<Note> for Standard {
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> Note {
        Note::new(random(), random())
    }
}

impl Default for Note {
    fn default() -> Self {
        Note {
            raw_note: RawNote::from('C'),
            pitch: Pitch::Natural,
        }
    }
}

impl Note {
    pub fn new(raw_note: char, pitch: Pitch) -> Self {
        let mut note = Note { raw_note: RawNote::from(raw_note), pitch };
        note.normalize();
        note
    }

    pub fn sharp_to_flat(&self) -> Note {
        if self.pitch != Pitch::Sharp {
            return self.clone();
        }
        let position = RAW_NOTE_MAP
            .iter()
            .position(|&x| x == self.raw_note.0)
            .unwrap();
        let new_position = (position + 1) % RAW_NOTE_MAP.len();
        Note::new(RAW_NOTE_MAP[new_position], Pitch::Flat)
    }

    pub fn flat_to_sharp(&self) -> Note {
        if self.pitch != Pitch::Flat {
            return self.clone();
        }
        let position = RAW_NOTE_MAP
            .iter()
            .position(|&x| x == self.raw_note.0)
            .unwrap();
        let new_position: usize;
        match position {
            0 => new_position = RAW_NOTE_MAP.len() - 1,
            _ => new_position = position - 1,
        }
        Note::new(RAW_NOTE_MAP[new_position], Pitch::Sharp)
    }

    // Adjust any half step pitched notes (e.g. B# -> C)
    pub fn normalize(&mut self) {
        let (n, p) = (self.raw_note.clone(), self.pitch.clone());
        match (n.0, p) {
            ('B', Pitch::Sharp) => {
                self.raw_note = RawNote::from('C');
                self.pitch = Pitch::Natural;
            }
            ('C', Pitch::Flat) => {
                self.raw_note = RawNote::from('B');
                self.pitch = Pitch::Natural;
            }
            ('E', Pitch::Sharp) => {
                self.raw_note = RawNote::from('F');
                self.pitch = Pitch::Natural;
            }
            ('F', Pitch::Flat) => {
                self.raw_note = RawNote::from('E');
                self.pitch = Pitch::Natural;
            }
            _ => ()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pitch {
    Natural,
    Sharp,
    Flat,
}

impl fmt::Display for Pitch {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Pitch::Flat => '♭',
            Pitch::Sharp => '♯',
            Pitch::Natural => '♮',
        };
        write!(f, "{}", symbol)
    }
}

impl Distribution<Pitch> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Pitch {
        match rng.gen_range(0..=2) {
            0 => Pitch::Natural,
            1 => Pitch::Sharp,
            2 => Pitch::Flat,
            _ => panic!("outside of pitch range")
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChordType {
    Major,
    Minor,
    MajorSeven,
    MinorSeven,
    DominantSeven,
    DominantSevenFlatNine,
    Diminished,
    MinorSevenFlatFive,
    Augmented,
    AugmentedSeven,
}

impl fmt::Display for ChordType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let chord = match self {
            ChordType::Major => "",
            ChordType::Minor => "m",
            ChordType::MajorSeven => "maj7",
            ChordType::MinorSeven => "m7",
            ChordType::DominantSeven => "7",
            ChordType::DominantSevenFlatNine => "7♭9",
            ChordType::Diminished => "dim",
            ChordType::MinorSevenFlatFive => "m7♭5",
            ChordType::Augmented => "+",
            ChordType::AugmentedSeven => "+7",
        };
        write!(f, "{}", chord)
    }
}

impl Distribution<ChordType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ChordType {
        match rng.gen_range(0..=2) {
            0 => ChordType::MajorSeven,
            1 => ChordType::MinorSeven,
            2 => ChordType::DominantSeven,
            _ => panic!("outside of chord range")
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chord {
    pub note: Note,
    pub chord_type: ChordType,
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.note.pitch {
            Pitch::Natural => write!(f, "{}{}", self.note.raw_note, self.chord_type),
            _ => write!(f, "{}{}{}", self.note.raw_note, self.note.pitch, self.chord_type),
        }
    }
}

impl Default for Chord {
    fn default() -> Self {
        Chord::new(
            Note::default(),
            ChordType::MajorSeven,
        )
    }
}

impl Chord {
    pub fn new(note: Note, chord_type: ChordType) -> Self {
        Chord {
            note,
            chord_type,
        }
    }

    pub fn random() -> Chord {
        let chord = Chord {
            note: random(),
            chord_type: random(),
        };
        chord
    }
}
