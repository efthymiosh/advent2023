use super::util;

use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric0, one_of, i64};
use nom::sequence::{delimited, separated_pair};
use nom::IResult;

#[derive(Debug)]
struct Move {
    dir: char,
    amt: i64,
    color: String,
}

fn parse_moves<'a>(input: &'a str) -> IResult<&'a str, Move> {
    let (remainder, ((dir, amt), colorstr)) = separated_pair(
        separated_pair(one_of("UDLR"), tag(" "), i64),
        tag(" "),
        delimited(tag("(#"), alphanumeric0, tag(")")),
    )(input)?;
    Ok((remainder, Move { dir, amt, color: colorstr.to_owned() }))
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let lines = util::parse_in_lines(&path)?;
    let mut edges = Vec::new();
    let mut state = (0, 0);
    for line in lines {
        let (_, m) = parse_moves(&line).unwrap();
        let diff = match m.dir {
            'L' => (-1, 0),
            'R' => (1, 0),
            'U' => (0, -1),
            'D' => (0, 1),
            _ => unreachable!(),
        };
        for _ in 0..m.amt {
            state = (state.0 + diff.0, state.1 + diff.1);
            edges.push(state);
        }
    }
    let area = util::math::polygon_area(edges.as_slice());
    let internal = util::math::picks_theorem(area, edges.as_slice());
    println!("Area: {}", internal + edges.len() as u64);
    Ok(())
}

fn pt2extract(m: Move) -> Move {
    Move {
        color: "".to_owned(),
        dir: match &m.color[5..] {
            "0" => 'R',
            "1" => 'D',
            "2" => 'L',
            "3" => 'U',
            _ => unreachable!(),
        },
        amt: i64::from_str_radix(&m.color[..5], 16).unwrap(),
    }
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let lines = util::parse_in_lines(&path)?;
    let mut edges = Vec::new();
    let mut state = (0, 0);
    for line in lines {
        let (_, m) = parse_moves(&line).unwrap();
        let m = pt2extract(m);
        let diff = match m.dir {
            'L' => (-1, 0),
            'R' => (1, 0),
            'U' => (0, -1),
            'D' => (0, 1),
            _ => unreachable!(),
        };
        for _ in 0..m.amt {
            state = (state.0 + diff.0, state.1 + diff.1);
            edges.push(state);
        }
    }
    let area = util::math::polygon_area(edges.as_slice());
    let internal = util::math::picks_theorem(area, edges.as_slice());
    println!("Area: {}", internal + edges.len() as u64);
    Ok(())
}
