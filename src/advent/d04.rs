use std::collections::{HashMap, HashSet};

use super::util;

use nom::bytes::complete::tag;
use nom::character::complete::u32;
use nom::multi::{many1, separated_list0};
use nom::sequence::{separated_pair, tuple};
use nom::IResult;

#[derive(Debug)]
struct Card {
    id: u32,
    winning: HashSet<u32>,
    played: HashSet<u32>,
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (remainder, (_space, id)) = separated_pair(tag("Card"), many1(tag(" ")), u32)(input)?;
    let (remainder, _) = tuple((tag(":"), many1(tag(" "))))(remainder)?;
    let (remainder, v_win) = separated_list0(many1(tag(" ")), u32)(remainder)?;
    let (remainder, _) = tuple((many1(tag(" ")), tag("|"), many1(tag(" "))))(remainder)?;
    let (remainder, v_played) = separated_list0(many1(tag(" ")), u32)(remainder)?;

    let winning: HashSet<u32> = v_win.into_iter().collect();
    let played: HashSet<u32> = v_played.into_iter().collect();

    Ok((
        remainder,
        Card {
            id,
            winning,
            played,
        },
    ))
}

fn parse_line(input: &str) -> Result<Card, Box<dyn std::error::Error + '_>> {
    let (remainder, card) = parse_card(input)?;
    if !remainder.is_empty() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Remainder remaining after parsing card: {}", remainder),
        )));
    }
    Ok(card)
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let lines = util::parse_in_lines(&path)?;
    let mut sum = 0;
    for line in lines {
        let card = parse_line(&line).unwrap();
        let amount = card.winning.intersection(&card.played).count();
        println!("{:?}", card);
        if amount > 0 {
            sum += 1 << (amount - 1)
        }
    }
    println!("Sum of points: {}", sum);
    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let lines = util::parse_in_lines(&path)?;
    let mut hm: HashMap<u32, (usize, u32)> = HashMap::new();
    let mut sum = 0;
    for line in lines {
        let card = parse_line(&line).unwrap();
        let amount = card.winning.intersection(&card.played).count();
        hm.insert(card.id, (amount, 1));
    }
    for i in 1..=hm.len() as u32 {
        let (amount, copies) = hm.remove(&i).unwrap();
        sum += copies;
        for _ in 0..copies {
            for j in (i + 1)..=(i + amount as u32) {
                if let Some((amount,copies)) = hm.get(&j) {
                    hm.insert(j, (*amount, *copies + 1));
                } else {
                    unreachable!();
                }
            }
        }
    }
    println!("Sum of points: {}", sum);
    Ok(())
}
