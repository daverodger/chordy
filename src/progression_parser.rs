#![allow(dead_code)]

use std::fs;
use std::str::FromStr;

use nom::{bytes::complete::take_while1, IResult};
use nom::branch::alt;
use nom::bytes::complete::tag;
use serde::{Deserialize, Serialize};
use serde_json::Result;

use crate::types::*;

#[derive(Serialize, Deserialize, Debug)]
struct RawProgression {
    name: String,
    tonality: String,
    progression: String,
}

fn parse_progression_file() -> Result<Vec<RawProgression>> {
    let file = fs::read("progressions.json").expect("progressions.json should exist");
    let progressions: Vec<RawProgression> = serde_json::from_slice(&file)?;
    Ok(progressions)
}

fn parse_scale_tone(input: &str) -> IResult<&str, ScaleTone> {
    let (input, sign) = alt((
        tag("flat"),
        tag("b"),
        tag(""),
    ))(input)?;

    let (input, tone) = take_while1(|c: char| c.is_alphabetic())(input)?;
    let scale_tone = sign.to_string() + tone;
    let res = ScaleTone::from_str(&scale_tone).unwrap();
    Ok((input, res))
}

fn parse_tone_with_timing(input: &str) -> IResult<&str, GenericChord> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chord_parser() {
        assert_eq!(parse_scale_tone("flatV"), Ok(("", ScaleTone::FlatFive)));
        assert_eq!(parse_scale_tone("bV"), Ok(("", ScaleTone::FlatFive)));
        assert_eq!(parse_scale_tone("VI"), Ok(("", ScaleTone::Six)));
        assert_eq!(parse_scale_tone("VI7(1)"), Ok(("7(1)", ScaleTone::Six)));
    }
}