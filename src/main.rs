use rand::{prelude::*, distributions::Standard};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq)]
enum RawNote {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl fmt::Display for RawNote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Distribution<RawNote> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RawNote {
        match rng.gen_range(0..=6) {
            0 => RawNote::A,
            1 => RawNote::B,
            2 => RawNote::C,
            3 => RawNote::D,
            4 => RawNote::E,
            5 => RawNote::F,
            6 => RawNote::G,
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
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Note {
        Note::new(random(), random())
    }
}

impl Default for Note {
    fn default() -> Self {
        Note {
            raw_note: RawNote::C,
            pitch: Pitch::Natural,
        }
    }
}

impl Note {
    fn new(raw_note: RawNote, pitch: Pitch) -> Self {
        let mut note = Note { raw_note, pitch };
        note.normalize();
        note
    }

    // Adjust any half step pitched notes (e.g. B# -> C)
    fn normalize(&mut self) {
        let (n, p) = (self.raw_note.clone(), self.pitch.clone());
        match (n, p) {
            (RawNote::B, Pitch::Sharp) => {
                self.raw_note = RawNote::C;
                self.pitch = Pitch::Natural;
            }
            (RawNote::C, Pitch::Flat) => {
                self.raw_note = RawNote::B;
                self.pitch = Pitch::Natural;
            }
            (RawNote::E, Pitch::Sharp) => {
                self.raw_note = RawNote::F;
                self.pitch = Pitch::Natural;
            }
            (RawNote::F, Pitch::Flat) => {
                self.raw_note = RawNote::E;
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
}

impl fmt::Display for ChordType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let chord = match self {
            ChordType::MajorSeven => "maj7",
            ChordType::MinorSeven => "m7",
            ChordType::DominantSeven => "7",
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
        let mut chord = Chord {
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

struct ChordProgression {
    root: Note,
    tones: Vec<GenericChord>,
    chords: Vec<Chord>,
}

impl ChordProgression {
    fn new(root: Note, tones: Vec<GenericChord>) -> Self {
        let chords = tones
            .iter()
            .map(|gc| gc.clone().into_chord(root.clone()))
            .collect();

        ChordProgression {
            root,
            tones,
            chords,
        }
    }
}

fn solve_interval(root: Note, tone: ScaleTone) -> Note {

    todo!()
}

fn generate_random_chord_chart(n: u8) -> String {
    let mut chord: Vec<Chord> = vec![];
    for _ in 0..n {
        chord.push(Chord::random());
    }

    let mut chart = String::new();
    chord
        .chunks(4)
        .for_each(|x| {
            chart.push('|');
            for c in x {
                chart.push_str(&format!("{:^10}|", c.to_string()))
            }
            chart.push_str("\n");
        }
        );
    chart
}

fn main() {
    let chart = generate_random_chord_chart(32);
    print!("{}", chart);
}
