use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use std::collections::HashMap;

use crate::advent::util::lcm;

struct Graph<'a> {
    map: HashMap<&'a str, (&'a str, &'a str)>,
}

fn parse_directions(input: &str) -> IResult<&str, String> {
    let (rem, ret) = alpha1(input)?;
    let (rem, _) = tag("\n\n")(rem)?;
    Ok((rem, ret.to_owned()))
}

fn parse_graph(input: &str) -> IResult<&str, Graph> {
    let (rem, v) = separated_list1(
        tag("\n"),
        tuple((alpha1, tag(" = ("), alpha1, tag(", "), alpha1, tag(")"))),
    )(input)?;
    let map: HashMap<&str, (&str, &str)> =
        v.iter().map(|(s, _, l, _, r, _)| (*s, (*l, *r))).collect();
    Ok((rem, Graph { map }))
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let input: String = std::fs::read_to_string(&path)?.trim().parse()?;

    let (rem, directions) = parse_directions(&input).unwrap();
    let (rem, graph) = parse_graph(rem).unwrap();

    if !rem.is_empty() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Remainder remaining after parsing game: {}", rem),
        )));
    }

    let mut pos = "AAA";
    let mut moves = 0;
    for (idx, c) in directions.chars().cycle().enumerate() {
        if pos == "ZZZ" {
            moves = idx;
            break;
        }
        if let Some((l, r)) = graph.map.get(pos) {
            println!("idx {} for {}, going {} on ({},{})", idx, pos, c, l, r);
            match c {
                'L' => pos = l,
                'R' => pos = r,
                _ => unreachable!(),
            };
        } else {
            unreachable!();
        }
    }
    println!("Moves to ZZZ: {}", moves);
    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let input: String = std::fs::read_to_string(&path)?.trim().parse()?;

    let (rem, directions) = parse_directions(&input).unwrap();
    let (rem, graph) = parse_graph(rem).unwrap();

    if !rem.is_empty() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Remainder remaining after parsing game: {}", rem),
        )));
    }

    let posv: Vec<&str> = graph
        .map
        .keys()
        .filter(|s| s.ends_with('A'))
        .map(|s| *s)
        .collect();
    let mut cadences: Vec<usize> = Vec::new();
    let mut offsets_to_z: Vec<usize> = Vec::new();
    for init in &posv {
        let mut pos = *init;
        let mut moves_to_z = Vec::new();
        for (idx, c) in directions.chars().cycle().enumerate() {
            if pos.ends_with("Z") {
                moves_to_z.push(idx);
                if moves_to_z.len() == 3 {
                    let first = *moves_to_z.first().unwrap();
                    let solvable =
                        moves_to_z.iter().fold(
                            true,
                            |state, &x| if !state { false } else { x % first == 0 },
                        );
                    if !solvable {
                        panic!("Problem not solvable with least-common-multiple");
                    }
                    offsets_to_z.push(first);
                    break;
                }
            }
            if let Some((l, r)) = graph.map.get(pos) {
                match c {
                    'L' => pos = l,
                    'R' => pos = r,
                    _ => unreachable!(),
                };
            } else {
                unreachable!();
            }
        }
    }
    println!("Moves to __Z: {:?}", offsets_to_z);
    let lcm = offsets_to_z.iter().fold(
        *(offsets_to_z.first().unwrap()) as i64, |s, &x| lcm(s, x as i64));
    println!("Least Common Multiple: {}", lcm);
    Ok(())
}
