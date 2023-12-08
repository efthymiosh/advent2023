use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use std::collections::HashMap;

struct Graph<'a> {
    map: HashMap<&'a str, (&'a str, &'a str)>,
}

fn parse_directions(input: &str) -> IResult<&str, String> {
    let (rem, ret) = alpha1(input)?;
    let (rem, _) = tag("\n\n")(rem)?;
    Ok((rem, ret.to_owned()))
}

fn parse_graph(input: &str) -> IResult<&str, Graph> {
    let (rem, v) = separated_list1(tag("\n"), tuple((alpha1, tag(" = ("), alpha1, tag(", "), alpha1, tag(")"))))(input)?;
    let init = v.first().unwrap().0;
    let map: HashMap<&str, (&str, &str)> = v.iter().map(|(s, _, l, _, r, _)| (*s, (*l, *r))).collect();
    Ok((rem, Graph {
        map,
    }))
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

    let mut posv: Vec<&str> = graph.map.keys().filter(|s| s.ends_with('A')).map(|s| *s).collect();
    let mut moves = 0;
    for (idx, c) in directions.chars().cycle().enumerate() {
        println!("{:?}", posv);
        if posv.iter().filter(|s| s.ends_with('Z')).count() == posv.len() {
            moves = idx;
            break;
        }
        let mut next: Vec<&str> = Vec::new();
        for &pos in &posv {
            if let Some((l, r)) = graph.map.get(pos) {
                match c {
                    'L' => next.push(l),
                    'R' => next.push(r),
                    _ => unreachable!(),
                };
            } else {
                unreachable!();
            }
        }
        posv = next;
    }
    println!("Moves to __Z: {}", moves);
    Ok(())
}
