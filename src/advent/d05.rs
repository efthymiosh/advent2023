use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, i64, multispace1};
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::IResult;
use std::collections::HashMap;
use std::ops::{Add, Sub};

#[derive(Debug)]
struct Range<T>
where
    T: Add<Output = T> + Sub<Output = T> + Default + Copy,
{
    start: T,
    length: T,
    transform: T,
}

impl<T: Add<Output = T> + Sub<Output = T> + Default + Copy + PartialOrd + PartialEq + Eq> Range<T> {
    fn new(start: T, length: T, transform: T) -> Range<T> {
        Range {
            start,
            length,
            transform,
        }
    }
    fn transform(self: &Self, n: T) -> Option<T> {
        if n >= self.start && (n - self.start) <= self.length {
            Some(n + self.transform)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct SeedMap {
    next: String,
    ranges: Vec<Range<i64>>,
}

impl SeedMap {
    fn transform(self: &Self, n: i64) -> i64 {
        for r in &self.ranges {
            if let Some(t) = r.transform(n) {
                return t;
            }
        }
        n
    }
//  fn merge(&mut self, mut other: SeedMap) {}
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<i64>> {
    let (rem, (_seeds, list)) =
        separated_pair(tag("seeds"), tag(": "), separated_list0(tag(" "), i64))(input)?;
    Ok((rem, list))
}

fn parse_seedmap(input: &str) -> IResult<&str, (String, SeedMap)> {
    let (rem, _) = tag("\n\n")(input)?;
    let (rem, (mapin, next)) = separated_pair(alpha1, tag("-to-"), alpha1)(rem)?;
    let (rem, _) = tag(" map:\n")(rem)?;
    let (rem, v) = (separated_list0(
        multispace1,
        separated_pair(separated_pair(i64, tag(" "), i64), tag(" "), i64),
    ))(rem)?;
    let maps = v
        .into_iter()
        .map(|((dest_start, source_start), range)| {
            Range::<i64>::new(source_start, range, dest_start - source_start)
        })
        .collect();
    Ok((
        rem,
        (
            mapin.to_owned(),
            SeedMap {
                next: next.to_owned(),
                ranges: maps,
            },
        ),
    ))
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let input: String = std::fs::read_to_string(&path)?.trim().parse()?;
    let mut seedmaps: HashMap<String, SeedMap> = HashMap::new();

    let (rem, seeds) = parse_seeds(&input).unwrap();
    let mut remainder = rem;
    while !remainder.is_empty() {
        let (rem, (id, seedmap)) = parse_seedmap(remainder).unwrap();
        remainder = rem;
        println!("{} {:?}", &id, &seedmap);
        seedmaps.insert(id, seedmap);
    }
    let initmap = seedmaps.get("seed").unwrap();
    let mut min = i64::max_value();
    for seed in seeds {
        let mut val = seed;
        let mut seedmap = initmap;
        loop {
            val = seedmap.transform(val);
            if seedmap.next == "location" {
                break;
            }
            seedmap = seedmaps.get(&seedmap.next).unwrap();
        }

        if val <= min {
            min = val;
        }
    }
    println!("Min location number: {}", min);
    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let _input: String = std::fs::read_to_string(&path)?.trim().parse()?;
    Ok(())
}
