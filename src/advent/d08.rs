use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use std::collections::HashMap;

struct Graph<'a> {
    map: HashMap<&'a str, (&'a str, &'a str)>,
    init: &'a str,
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
    let init = v.first().unwrap().0;
    let map: HashMap<&str, (&str, &str)> =
        v.iter().map(|(s, _, l, _, r, _)| (*s, (*l, *r))).collect();
    Ok((rem, Graph { map, init }))
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

    let mut moves = 0;
    let intmap: HashMap<&str, usize> = graph
        .map
        .keys()
        .enumerate()
        .map(|(idx, s)| (*s, idx))
        .collect();
    let mut arr: Vec<(usize, usize)> = Vec::new();
    graph.map.keys().for_each(|&key| {
        let s = intmap.get(key).unwrap();
        let (ls, rs) = graph.map.get(key).unwrap();
        let l = intmap.get(ls).unwrap();
        let r = intmap.get(rs).unwrap();
        arr.insert(*s, (*l, *r));
    });

    let mut pos = *intmap.get("AAA").unwrap();
    let zzz = *intmap.get("ZZZ").unwrap();

    for (idx, c) in directions.chars().cycle().enumerate() {
        if pos == zzz {
            moves = idx;
            break;
        }
        let (l, r) = arr[pos];
        println!("idx {} for {}, going {} on ({},{})", idx, pos, c, l, r);
        match c {
            'L' => pos = l,
            'R' => pos = r,
            _ => unreachable!(),
        };
    }
    println!("Moves to ZZZ: {}", moves);
    Ok(())
}

pub fn pt2(_path: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Min location number: {}", 1);
    Ok(())
}
