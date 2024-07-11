use crate::types::*;

const CHORD_MAX_LEN: usize = 6;

pub fn generate_random_chord_chart(n: u8) -> String {
    let mut chords: Vec<Chord> = vec![];
    for _ in 0..n {
        chords.push(Chord::random());
    }
    format_chord_chart(chords)
}

pub fn format_chord_chart(chord: Vec<Chord>) -> String {
    let mut chart = String::new();
    chord.chunks(4).for_each(|x| {
        chart.push('|');
        for c in x {
            chart.push_str(&format!(" {:<1$} |", c.to_string(), CHORD_MAX_LEN))
        }
        chart.push_str("\n");
    }
    );
    let chart = chart.trim_end_matches("\n");
    chart.to_string()
}

pub fn format_synced_chord_chart(chords: Vec<ChordSynced>) -> String {
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
