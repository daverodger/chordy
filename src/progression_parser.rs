#![allow(dead_code)]

use std::fs;
use std::str::FromStr;

use nom::{bytes::complete::take_while1, IResult};
use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_till1};
use nom::character::complete::multispace0;
use nom::combinator::{iterator, opt};
use nom::sequence::{terminated, Tuple};
use serde::{Deserialize, Serialize};
use serde_json::Result;

use crate::helpers::is_roman_tone;
use crate::types::*;

#[derive(Serialize, Deserialize, Debug)]
struct RawProgression {
    name: String,
    tonality: String,
    progression: String,
}

pub fn parse_progression_file() -> Result<Vec<GenericProgression>> {
    let file = fs::read("progressions.json").expect("progressions.json should exist");
    let progressions: Vec<RawProgression> = serde_json::from_slice(&file)?;
    let progressions = progressions
        .into_iter()
        .map(|p| GenericProgression {
            name: p.name,
            tonality: Tonality::from_str(&p.tonality).expect("Unable to parse tonality field"),
            progression: parse_chord_prog(&p.progression),
        })
        .collect::<Vec<GenericProgression>>();
    Ok(progressions)
}

fn parse_scale_tone(input: &str) -> IResult<&str, ScaleTone> {
    let (input, sign) = alt((
        tag("flat"),
        tag("b"),
        tag(""),
    ))(input)?;

    let (input, tone) = take_while1(|c: char| is_roman_tone(c))(input)?;
    let scale_tone = sign.to_string() + tone;
    let res = ScaleTone::from_str(&scale_tone).unwrap();
    Ok((input, res))
}

fn parse_chord_type(input: &str) -> IResult<&str, ChordType> {
    let (input, chord) = take_till1(|c| c == '(' || c == ' ')(input)?;
    let res = ChordType::from_str(chord).unwrap();
    Ok((input, res))
}

fn parse_beat_count(input: &str) -> IResult<&str, u8> {
    let (input, start) = opt(tag("("))(input)?;
    if start.is_none() {
        return Ok((input, 4));
    }
    let (input, beat) = take(1usize)(input)?;
    let (input, _) = tag(")")(input)?;
    let beat = u8::from_str_radix(beat, 10).unwrap_or(4);
    Ok((input, beat))
}

fn parse_synced_generic(input: &str) -> IResult<&str, GenericChordSynced> {
    let (input, (tone, ctype, beat)) = (parse_scale_tone, parse_chord_type, parse_beat_count).parse(input)?;
    let res = GenericChordSynced {
        generic_chord: GenericChord { scale_tone: tone, chord_type: ctype },
        beats: beat,
    };
    Ok((input, res))
}

fn parse_chord_prog(input: &str) -> Vec<GenericChordSynced> {
    let mut it = iterator(input, terminated(parse_synced_generic, multispace0));
    let parsed: Vec<GenericChordSynced> = it.collect();
    parsed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tone_parser() {
        assert_eq!(parse_scale_tone("flatV"), Ok(("", ScaleTone::FlatFive)));
        assert_eq!(parse_scale_tone("bV"), Ok(("", ScaleTone::FlatFive)));
        assert_eq!(parse_scale_tone("VI"), Ok(("", ScaleTone::Six)));
        assert_eq!(parse_scale_tone("VI7(1)"), Ok(("7(1)", ScaleTone::Six)));
        assert_eq!(parse_scale_tone("I"), Ok(("", ScaleTone::One)));
        assert_eq!(parse_scale_tone("II"), Ok(("", ScaleTone::Two)));
        assert_eq!(parse_scale_tone("V"), Ok(("", ScaleTone::Five)));
    }

    #[test]
    fn chord_parser() {
        assert_eq!(parse_chord_type("maj7(1)"), Ok(("(1)", ChordType::MajorSeven)));
        assert_eq!(parse_chord_type("min7 "), Ok((" ", ChordType::MinorSeven)));
        assert_eq!(parse_chord_type("7"), Ok(("", ChordType::DominantSeven)));
    }

    #[test]
    fn beat_parser() {
        assert_eq!(parse_beat_count("(4) "), Ok((" ", 4)));
        assert_eq!(parse_beat_count(" Vmaj7"), Ok((" Vmaj7", 4)));
    }

    #[test]
    fn sgc_parser() {
        let chord = GenericChordSynced {
            generic_chord: GenericChord {
                scale_tone: ScaleTone::FlatFive,
                chord_type: ChordType::DominantSevenFlatNine,
            },
            beats: 4,
        };
        assert_eq!(parse_synced_generic("flatV7b9 V7"), Ok((" V7", chord)));
    }

    #[test]
    fn prog_parser() {
        let chord1 = GenericChordSynced {
            generic_chord: GenericChord {
                scale_tone: ScaleTone::FlatFive,
                chord_type: ChordType::DominantSevenFlatNine,
            },
            beats: 4,
        };
        let chord2 = GenericChordSynced {
            generic_chord: GenericChord {
                scale_tone: ScaleTone::One,
                chord_type: ChordType::DominantSeven,
            },
            beats: 4,
        };
        assert_eq!(parse_chord_prog("flatV7b9 I7(4)"), vec![chord1, chord2]);

        let chord1 = GenericChordSynced {
            generic_chord: GenericChord {
                scale_tone: ScaleTone::Two,
                chord_type: ChordType::MinorSeven,
            },
            beats: 4,
        };
        let chord2 = GenericChordSynced {
            generic_chord: GenericChord {
                scale_tone: ScaleTone::Five,
                chord_type: ChordType::DominantSeven,
            },
            beats: 4,
        };
        let chord3 = GenericChordSynced {
            generic_chord: GenericChord {
                scale_tone: ScaleTone::One,
                chord_type: ChordType::MajorSeven,
            },
            beats: 4,
        };
        assert_eq!(parse_chord_prog("IImin7(4) V7(4) Imaj7(4)"), vec![chord1, chord2, chord3]);
    }
}