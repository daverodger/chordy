#![allow(dead_code)]

use chord_generator::charts::*;
use chord_generator::generic_progressions::*;
use chord_generator::helpers::solve_interval;
use chord_generator::types::*;

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