#![allow(dead_code)]

mod generic_progressions;

use rand::{prelude::*, distributions::Standard};
use std::fmt;
use std::fmt::Formatter;
use generic_progressions::*;

const CHORD_MAX_LEN: usize = 6;

static NOTE_MAP: [Note; 12] = [
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

const RAW_NOTE_MAP: [char; 7] = ['A', 'B', 'C', 'D', 'E', 'F', 'G'];

const TONE_MAP: [ScaleTone; 12] = [
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

#[derive(Debug, Clone, PartialEq, Copy)]
struct RawNote(char);

impl RawNote {
    fn from(note: char) -> Self {
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
struct Note {
    raw_note: RawNote,
    pitch: Pitch,
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
    fn new(raw_note: char, pitch: Pitch) -> Self {
        let mut note = Note { raw_note: RawNote::from(raw_note), pitch };
        note.normalize();
        note
    }

    fn sharp_to_flat(&self) -> Note {
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

    fn flat_to_sharp(&self) -> Note {
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
    fn normalize(&mut self) {
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
enum Pitch {
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
enum ChordType {
    MajorSeven,
    MinorSeven,
    DominantSeven,
    DominantSevenFlatNine,
    Diminished,
}

impl fmt::Display for ChordType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let chord = match self {
            ChordType::MajorSeven => "maj7",
            ChordType::MinorSeven => "m7",
            ChordType::DominantSeven => "7",
            ChordType::DominantSevenFlatNine => "7♭9",
            ChordType::Diminished => "dim"
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
struct Chord {
    note: Note,
    chord_type: ChordType,
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
    fn new(note: Note, chord_type: ChordType) -> Self {
        Chord {
            note,
            chord_type,
        }
    }

    fn random() -> Chord {
        let chord = Chord {
            note: random(),
            chord_type: random(),
        };
        chord
    }
}

#[derive(Debug, PartialEq, Clone)]
enum ScaleTone {
    One,
    FlatTwo,
    Two,
    FlatThree,
    Three,
    Four,
    FlatFive,
    Five,
    FlatSix,
    Six,
    FlatSeven,
    Seven,
}

#[derive(Debug, PartialEq, Clone)]
struct GenericChord {
    scale_tone: ScaleTone,
    chord_type: ChordType,
}

impl GenericChord {
    fn new(scale_tone: ScaleTone, chord_type: ChordType) -> Self {
        GenericChord { scale_tone, chord_type }
    }

    fn into_chord(self, root: Note) -> Chord {
        let note = solve_interval(root.clone(), self.scale_tone);
        Chord::new(note, self.chord_type)
    }
}

#[derive(Debug, PartialEq, Clone)]
struct GenericChordSynced {
    generic_chord: GenericChord,
    beats: u8,
}

#[derive(Debug, PartialEq, Clone)]
struct ChordSynced {
    chord: Chord,
    beats: u8,
}

struct ChordProgression {
    root: Note,
    chords: Vec<ChordSynced>,
    key: Key,
}

impl ChordProgression {
    fn new<I>(root: &Note, key: &Key, tones: &I) -> Self
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

    fn auto_convert_flats(&mut self) {
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
enum Key {
    Major,
    Minor,
}

fn solve_interval(root: Note, tone: ScaleTone) -> Note {
    let root = root.flat_to_sharp();
    let position = NOTE_MAP.iter().position(|n| *n == root).unwrap();

    let shift = TONE_MAP.iter().position(|t| *t == tone).unwrap();
    let new_position = (position + shift) % NOTE_MAP.len();
    NOTE_MAP.get(new_position).unwrap().clone()
}

fn generate_random_chord_chart(n: u8) -> String {
    let mut chords: Vec<Chord> = vec![];
    for _ in 0..n {
        chords.push(Chord::random());
    }
    format_chord_chart(chords)
}

fn format_chord_chart(chord: Vec<Chord>) -> String {
    let mut chart = String::new();
    chord
        .chunks(4)
        .for_each(|x| {
            chart.push('|');
            for c in x {
                chart.push_str(&format!(" {:<1$} |", c.to_string(), CHORD_MAX_LEN))
            }
            chart.push_str("\n");
        }
        );
    chart
}

fn format_synced_chord_chart(chords: Vec<ChordSynced>) -> String {
    // Beat value determines max_width
    let mut min = 4;
    for c in &chords {
        if c.beats < min {
            min = c.beats;
            if min == 3 || min == 1 { break; }
        }
    }
    let max_width = match min {
        4 => CHORD_MAX_LEN,
        1 | 3 => CHORD_MAX_LEN * 4 + 6,
        2 => CHORD_MAX_LEN * 2 + 2,
        _ => panic!("error formatting width")
    };

    let mut chart = String::from("| ");
    let mut beats = 0;
    let mut bars = 0;
    for c in chords {
        beats += c.beats;
        match c.beats {
            1 => chart.push_str(&format!("{:<1$}", c.chord.to_string(), max_width / 4)),
            2 => chart.push_str(&format!("{:<1$}", c.chord.to_string(), max_width / 2)),
            3 => chart.push_str(&format!("{:<1$}", c.chord.to_string(), (max_width * 2) / 3)),
            4 => chart.push_str(&format!("{:<1$}", c.chord.to_string(), max_width)),
            _ => panic!("unimplemented beat value")
        }
        if beats == 4 {
            bars += 1;
            beats = 0;
            if bars == 4 {
                bars = 0;
                chart.push_str(" |\n| ");
            } else {
                chart.push_str(" | ");
            }
        } else if beats > 4 {
            panic!("unimplemented time signature")
        }
    }
    let chart = chart.trim_end_matches("\n| ");
    chart.to_string()
}

fn main() {
    for note in &NOTE_MAP {
        let progression = ChordProgression::new(note, &Key::Major, &BLUES_BASIE);
        let chart = format_synced_chord_chart(progression.chords);
        println!("{}", chart);
        println!()
    }

    // let chart = generate_random_chord_chart(32);
    // println!("{}", chart);
}

#[cfg(test)]
mod tests {
    use crate::{Note, Pitch, ScaleTone, solve_interval};

    #[test]
    fn interval_solver() {
        let note = Note::new('D', Pitch::Natural);

        let five = ScaleTone::Five;
        let new = solve_interval(note.clone(), five);
        assert_eq!(new, Note::new('A', Pitch::Natural));

        let flat_two = ScaleTone::FlatTwo;
        let new = solve_interval(note.clone(), flat_two);
        assert_eq!(new, Note::new('D', Pitch::Sharp));

        let seven = ScaleTone::Seven;
        let new = solve_interval(note.clone(), seven);
        assert_eq!(new, Note::new('C', Pitch::Sharp));
    }
}