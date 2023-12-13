use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, i64, multispace1};
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::IResult;
use num_traits::PrimInt;
use std::collections::HashMap;
use std::fmt::Debug;

use crate::advent::util;

#[derive(Debug)]
struct Range<T>
where
    T: PrimInt+Debug,
{
    start: T,
    length: T,
    transform: T,
}

impl<T: PrimInt+Debug> Clone for Range<T> {
    fn clone(&self) -> Self {
        Range {
            start: self.start,
            length: self.length,
            transform: self.transform,
        }
    }
}

impl<T: PrimInt+Debug> Copy for Range<T> {}

impl<T: PrimInt+Debug> Range<T> {
    fn new(start: T, length: T, transform: T) -> Range<T> {
        Range {
            start,
            length,
            transform,
        }
    }
    fn transform(self: &Self, n: T) -> Option<T> {
        if n >= self.start && (n - self.start) < self.length {
            Some(n + self.transform)
        } else {
            None
        }
    }
    fn split_against(self: Self, other: &Range<T>) -> Option<(Range<T>, Range<T>)> {
        if !self.intersects(other){
            return None;
        }
        let split: T;
        if self.start <= other.start {
            split = other.start;
        } else {
            split = other.start + other.length;
        }
        Some((
            Range::new(self.start, split - self.start, self.transform),
            Range::new(split + T::one(), self.length - (split - self.start) - T::one(), self.transform),
        ))
    }
    fn intersects(self: &Self, other: &Range<T>) -> bool {
        !(self.start + self.length <= other.start || other.start + other.length <= self.start)
    }

    fn passthrough(self: Self, other: &Range<T>) -> Vec<Range<T>> {
        if self.start + self.length < other.start || other.start + other.length < self.start {
            return vec![self];
        }
        let mut v = Vec::new();
        // find intersection
        let mut intersection: Range<T> = Range {
            start: T::zero(),
            length: T::zero(),
            transform: self.transform + other.transform,
        };
        if self.start < other.start {
            intersection.start = other.start;
            v.push(Range{
                start: self.start,
                length: other.start - self.start,
                transform: self.transform,
            });
        } else {
            intersection.start = self.start;
        }
        if self.start + self.length <= other.start + other.length {
            intersection.length = self.start + self.length - intersection.start;
        } else {
            intersection.length = other.start + other.length - intersection.start;
            v.push(Range{
                start: other.start + other.length,
                length: self.start + self.length - (other.start + other.length),
                transform: other.transform,
            });
        }
        v.push(intersection);
        v
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

    fn intersect_range(self: &Self, range: Range<i64>) -> Vec<Range<i64>> {
        let mut v = Vec::new();
        let intersecting_ranges: Vec<&Range<i64>> = self.ranges.iter().filter(|r| r.intersects(&range)).collect();
        match intersecting_ranges.len() {
            0 => {v.push(range)},
            1 => {
                let r = intersecting_ranges[0];
                let mut vr = range.passthrough(r);
                v.append(&mut vr);
            }
            _ => {
                for r in intersecting_ranges {
                    if let Some((r1,r2)) = range.split_against(r) {
                        v.append(&mut self.intersect_range(r1));
                        v.append(&mut self.intersect_range(r2));
                    }
                }
            }
        };
        v
    }
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<i64>> {
    let (rem, (_seeds, list)) =
        separated_pair(tag("seeds"), tag(": "), separated_list0(tag(" "), i64))(input)?;
    Ok((rem, list))
}

fn parse_seeds_pt2(input: &str) -> IResult<&str, Vec<Range<i64>>> {
    let (rem, (_seeds, list)) = separated_pair(
        tag("seeds"),
        tag(": "),
        separated_list0(tag(" "), separated_pair(i64, tag(" "), i64)),
    )(input)?;
    Ok((
        rem,
        list.into_iter()
            .map(|(x, y)| Range::new(x, y, 0))
            .collect(),
    ))
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
    let input: String = std::fs::read_to_string(&path)?.trim().parse()?;
    let mut seedmaps: HashMap<String, SeedMap> = HashMap::new();

    let (rem, seedranges) = parse_seeds_pt2(&input).unwrap();
    let mut remainder = rem;
    while !remainder.is_empty() {
        let (rem, (id, seedmap)) = parse_seedmap(remainder).unwrap();
        remainder = rem;
        seedmaps.insert(id, seedmap);
    }
    let mut min = Range::new(i64::max_value(), 0, 0);
    let initmap = seedmaps.get("seed").unwrap();
    for seedrange in seedranges {
        let mut vals = vec![seedrange];
        let mut seedmap = initmap;
        loop {
            println!("Seedrange {:?}", vals);
            println!("THROUGH {:?}", seedmap.ranges);
            let mut newvals = Vec::new();
            while let Some(val) = vals.pop() {
                let mut newval = seedmap.intersect_range(val);
                newvals.append(&mut newval);
            }
            if !newvals.is_empty() {
                vals = newvals;
            }
            if seedmap.next == "location" {
                break;
            }
            seedmap = seedmaps.get(&seedmap.next).unwrap();
        }

        for r in vals {
            if r.start + r.transform < min.start + min.transform {
                min = r;
            }
        }
        util::pause();
    }
    println!("Min location number: {:?}", min.start + min.transform);
    Ok(())
}
