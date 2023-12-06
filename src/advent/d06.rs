use std::collections::{HashMap, HashSet};

use super::util;

use nom::bytes::complete::tag;
use nom::character::complete::u64;
use nom::multi::{many1, separated_list0};
use nom::sequence::{separated_pair, tuple};
use nom::IResult;

fn parse<'a>(what: &str, input: &'a str) -> IResult<&'a str, Vec<u64>> {
    let (remainder, _) = tuple((tag(what), many1(tag(" "))))(input)?;
    let (remainder, v) = separated_list0(many1(tag(" ")), u64)(remainder)?;
    let (remainder, _) = tag("\n")(remainder)?;

    Ok((remainder, v))
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn parse_input(input: &str) -> Result<Vec<Race>, Box<dyn std::error::Error + '_>> {
    let (remainder, time) = parse("Time:", input)?;
    let (remainder, distance) = parse("Distance:", remainder)?;
    if !remainder.is_empty() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Remainder remaining after parsing card: {}", remainder),
        )));
    }
    Ok(time
        .into_iter()
        .zip(distance)
        .map(|(time, distance)| Race { time, distance })
        .collect())
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let input: String = std::fs::read_to_string(&path)?.parse()?;
    let races = parse_input(&input).unwrap();
    let mut res = 1;
    for race in races {
        let mut success_tries = 0;
        for hold in 0..=race.time {
            let dist = (race.time - hold) * hold;
            println!("Held {} moved for {}", hold, dist);
            if dist > race.distance {
                success_tries += 1;
            }
        }
        res *= success_tries;
    }
    println!("Mul of values: {:#?}", res);
    Ok(())
}

fn parse_pt2<'a>(what: &str, input: &'a str) -> IResult<&'a str, u64> {
    let (remainder, _) = (tag(what))(input)?;
    let (remainder, (v, _)) = tuple((u64, tag("\n")))(remainder)?;

    Ok((remainder, v))
}

fn parse_input_pt2(input: &str) -> Result<Race, Box<dyn std::error::Error + '_>> {
    let (remainder, time) = parse_pt2("Time:", input)?;
    let (remainder, distance) = parse_pt2("Distance:", remainder)?;
    if !remainder.is_empty() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Remainder remaining after parsing card: {}", remainder),
        )));
    }
    Ok(Race { time, distance })
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let input: String = std::fs::read_to_string(&path)?.parse()?;
    let parsedinput: String = input.chars().filter(|&c| !(c == ' ')).collect();
    let race = parse_input_pt2(&parsedinput).unwrap();
    let mut res = 1;
    let mut first_success = 0;
    for hold in 0..=race.time {
        let dist = (race.time - hold) * hold;
        if dist > race.distance {
            first_success = hold;
            break;
        }
    }
    // found first success, find last
    let mut last_success = 0;
    for hold in (0..=race.time).rev() {
        let dist = (race.time - hold) * hold;
        if dist > race.distance {
            last_success = hold;
            break;
        }
    }
    res = last_success - first_success + 1;
    println!("Mul of values: {:#?}", res);
    Ok(())
}
