use crate::types::*;

pub fn solve_interval(root: Note, tone: ScaleTone) -> Note {
    let root = root.flat_to_sharp();
    let position = NOTE_MAP.iter().position(|n| *n == root).unwrap();

    let shift = TONE_MAP.iter().position(|t| *t == tone).unwrap();
    let new_position = (position + shift) % NOTE_MAP.len();
    NOTE_MAP.get(new_position).unwrap().clone()
}
