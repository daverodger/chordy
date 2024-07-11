#![allow(dead_code)]

use sarge::prelude::*;
use chord_generator::charts::generate_random_chord_chart;

sarge! {
    Args,
    #ok 'p' progression: String,
    'r' random: bool,
    #ok 'k' key: String,
    #ok 'b' bars: u8,
}

fn main() {
    let (args, _remainder) = Args::parse().expect("failed to parse arguments");
    match (args.progression, args.key, args.bars, args.random) {
        (None, None, None, false) => println!("{}", generate_random_chord_chart(32)),
        (None, key, None, true) => {
            todo!()
        }
        _ => ()
    }


}