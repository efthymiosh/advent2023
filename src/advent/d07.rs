use std::cmp::Ordering;
use std::collections::HashMap;

use super::util;

use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, u32};
use nom::sequence::separated_pair;
use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    hand: Vec<u32>,
    hand_alt: Vec<(u32, u32)>,
    class: u32,
    class_alt: u32,
    bid: u32,
}

impl Hand {
    fn new(hand: Vec<u32>, bid: u32) -> Self {
        if hand.len() != 5 {
            panic!("Not supposed to receive hands with something other than 5 cards");
        }

        let mut hm: HashMap<u32, u32> = HashMap::new();
        for x in &hand {
            let mut v = 0;
            if let Some(vin) = hm.remove(&x) {
                v = vin;
            }
            hm.insert(*x, v + 1);
        }

        let mut hand_alt: Vec<(u32, u32)> = hm.into_iter().map(|(k, v)| (v, k)).collect();
        hand_alt.sort_by(
            |(ak, av), (bk, bv)| {
                if ak == bk {
                    av.cmp(bv)
                } else {
                    ak.cmp(bk)
                }
            },
        );

        let hand_alt: Vec<(u32, u32)> = hand_alt.into_iter().rev().collect();
        let class = match hand_alt[..] {
            [(5,_)] => 7,
            [(4,_), (1,_)] => 6,
            [(3,_), (2,_)] => 5,
            [(3,_), (1,_), (1,_)] => 4,
            [(2,_), (2,_), (1,_)] => 3,
            [(2,_), (1,_), (1,_), (1,_)] => 2,
            _ => 1,
        };

        let class_alt = match hand_alt[..] {
            [(5,_)] => 7,
            [(4,_), (1,0)] => 7,
            [(4,0), (1,_)] => 7,
            [(4,_), (1,_)] => 6,
            [(3,0), (2,_)] => 7,
            [(3,_), (2,0)] => 7,
            [(3,_), (2,_)] => 5,
            [(3,0), (1,_), (1,_)] => 6,
            [(3,_), (1,_), (1,0)] => 6,
            [(3,_), (1,_), (1,_)] => 4,
            [(2,_), (2,0), (1,_)] => 6,
            [(2,_), (2,_), (1,0)] => 5,
            [(2,_), (2,_), (1,_)] => 3,
            [(2,0), (1,_), (1,_), (1,_)] => 4,
            [(2,_), (1,_), (1,_), (1,0)] => 4,
            [(2,_), (1,_), (1,_), (1,_)] => 2,
            [(1,_), (1,_), (1,_), (1,_), (1,0)] => 2,
            _ => 1,
        };

        Hand { bid, class, class_alt, hand, hand_alt }
    }

    fn cmp_alt(&self, other: &Self) -> Ordering {
        if self.class_alt > other.class_alt {
            Ordering::Greater
        } else if self.class_alt < other.class_alt {
            Ordering::Less
        } else {
            self.hand.iter().zip(other.hand.iter()).fold(
                Ordering::Equal,
                |s, (a, b)| {
                    if s != Ordering::Equal {
                        return s;
                    } else if a == b {
                        Ordering::Equal
                    } else {
                        a.cmp(b)
                    }
                },
            )
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.class > other.class {
            Some(Ordering::Greater)
        } else if self.class < other.class {
            Some(Ordering::Less)
        } else {
            self.hand.iter().zip(other.hand.iter()).fold(
                Some(Ordering::Equal),
                |s, (a,b)| {
                    if s != Some(Ordering::Equal) {
                        return s;
                    }
                    if a == b {
                        Some(Ordering::Equal)
                    } else {
                        a.partial_cmp(b)
                    }
                },
            )
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_hand(input: &str, joker: bool) -> IResult<&str, Hand> {
    let (remainder, (str, bid)) = separated_pair(alphanumeric1, tag(" "), u32)(input)?;
    let hand: Vec<u32> = str
        .chars()
        .map(|c| {
            if c.is_ascii_digit() {
                c.to_digit(10).unwrap()
            } else {
                match c {
                    'T' => 10,
                    'J' => if joker {0} else {11},
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => unreachable!(),
                }
            }
        })
        .collect();
    Ok((remainder, Hand::new(hand, bid)))
}

fn parse(input: &str, joker: bool) -> Result<Hand, Box<dyn std::error::Error + '_>> {
    let (remainder, hand) = parse_hand(input, joker)?;
    if !remainder.is_empty() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Remainder remaining after parsing game: {}", remainder),
        )));
    } else {
        return Ok(hand);
    }
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let lines = util::parse_in_lines(&path)?;
    let mut v = Vec::new();
    for line in lines {
        let hand = parse(&line, false).unwrap();
        v.push(hand);
    }
    v.sort();
    let sum = v
        .iter()
        .enumerate()
        .fold(0, |s, (idx, h)| s + h.bid * (idx as u32 + 1));
    for (idx, hand) in v.iter().enumerate() {
        println!("{} {:?}", idx, hand);
    }
    println!("Sum: {}", sum);
    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let lines = util::parse_in_lines(&path)?;
    let mut v = Vec::new();
    for line in lines {
        let hand = parse(&line, true).unwrap();
        v.push(hand);
    }
    v.sort_by(Hand::cmp_alt);
    let sum = v
        .iter()
        .enumerate()
        .fold(0, |s, (idx, h)| s + h.bid * (idx as u32 + 1));
    for (idx, hand) in v.iter().enumerate() {
        println!("{} class {} | {:?}", idx + 1, hand.class_alt, hand.hand);
    }
    println!("Sum: {}", sum);
    Ok(())
}
