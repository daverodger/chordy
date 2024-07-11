#![allow(dead_code)]

use rand::random;
use sarge::prelude::*;

use chord_generator::charts::{format_synced_chord_chart, generate_random_chord_chart};
use chord_generator::helpers::{find_progression, random_progression};
use chord_generator::progression_parser::parse_progression_file;
use chord_generator::types::{ChordProgression, Note};

sarge! {
    Args,
    #ok 'p' progression: String,
    'r' random: bool,
    #ok 'k' key: String,
    #ok 'b' bars: u8,
}

fn main() {
    let (args, _remainder) = Args::parse().expect("failed to parse arguments");
    let progressions = parse_progression_file().expect("failed to parse progression file");
    let mut chart = String::new();
    match (args.progression, args.key, args.bars, args.random) {
        (None, None, bars, false) => {
            chart = generate_random_chord_chart(bars.unwrap_or(24));
        },
        (None, key, None, true) => {
            let p = random_progression(progressions).expect("progression list empty");
            match key {
                Some(k) => {
                    let key = Note::from_str(&k).expect("failed to parse key");
                    let prog = ChordProgression::from_generic(&key, &p);
                    chart = format_synced_chord_chart(prog.chords);
                }
                None => {
                    let prog = ChordProgression::from_generic(&random(), &p);
                    chart = format_synced_chord_chart(prog.chords);
                }
            }
        }
        (Some(p), key, None, false) => {
            let prog = find_progression(progressions, &p);
            match prog {
                None => chart = format!("could not find progression: {}", &p),
                Some(prog) => {
                    let note: Note;
                    match key {
                        None => note = random(),
                        Some(k) => note = Note::from_str(&k).unwrap(),
                    }
                    chart = format_synced_chord_chart(ChordProgression::from_generic(&note, &prog).chords);
                }
            }
        }
        _ => ()
    }
    println!("{}", chart);
}