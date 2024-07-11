use rand::{Rng, thread_rng};
use crate::types::*;

pub fn solve_interval(root: Note, tone: ScaleTone) -> Note {
    let root = root.flat_to_sharp();
    let position = NOTE_MAP.iter().position(|n| *n == root).unwrap();

    let shift = TONE_MAP.iter().position(|t| *t == tone).unwrap();
    let new_position = (position + shift) % NOTE_MAP.len();
    NOTE_MAP.get(new_position).unwrap().clone()
}

pub fn is_roman_tone(c: char) -> bool {
    match c {
        'I' | 'V' => true,
        _ => false
    }
}

pub fn find_progression(progressions: Vec<GenericProgression>, name: &str) -> Option<GenericProgression> {
    for p  in progressions {
        if p.name.eq_ignore_ascii_case(name) {
            return Some(p)
        }
    }
    None
}

pub fn random_progression(progressions: Vec<GenericProgression>) -> Option<GenericProgression> {
    if progressions.is_empty() {return None}
    let r = thread_rng().gen_range(0..progressions.len());
    Some(progressions.get(r).expect("should be inside range bounds").clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interval_solver() {
        let note = Note::new(RawNote::from('D'), Pitch::Natural);

        let five = ScaleTone::Five;
        let new = solve_interval(note.clone(), five);
        assert_eq!(new, Note::new(RawNote::from('A'), Pitch::Natural));

        let flat_two = ScaleTone::FlatTwo;
        let new = solve_interval(note.clone(), flat_two);
        assert_eq!(new, Note::new(RawNote::from('D'), Pitch::Sharp));

        let seven = ScaleTone::Seven;
        let new = solve_interval(note.clone(), seven);
        assert_eq!(new, Note::new(RawNote::from('C'), Pitch::Sharp));
    }
}
