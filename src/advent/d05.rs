use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, i64, multispace1};
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::IResult;
use num_traits::PrimInt;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Debug, Display};

#[derive(Debug)]
struct Range<T>
where
    T: PrimInt+Debug,
{
    start: T,
    end: T,
    transform: T,
}

impl<T: PrimInt+Debug> PartialEq for Range<T> {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start
    }
}

impl<T: PrimInt+Debug> Eq for Range<T> {}

impl<T: PrimInt+Debug> PartialOrd for Range<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.start.partial_cmp(&other.start)
    }
}

impl<T: PrimInt+Debug> Ord for Range<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl<T: PrimInt+Debug> Clone for Range<T> {
    fn clone(&self) -> Self {
        Range {
            start: self.start,
            end: self.end,
            transform: self.transform,
        }
    }
}

impl <T: PrimInt+Debug+Display> Display for Range<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:12}, {:12}]({:11})", self.start, self.end, self.transform)
    }
}

impl<T: PrimInt+Debug> Copy for Range<T> {}

impl<T: PrimInt+Debug+Display> Range<T> {
    fn new(start: T, end: T, transform: T) -> Range<T> {
        Range {
            start,
            end,
            transform,
        }
    }
    fn transform(self: &Self, n: T) -> Option<T> {
        if n >= self.start && n <= self.end {
            Some(n + self.transform)
        } else {
            None
        }
    }
    #[inline]
    fn intersects(self: &Self, other: &Range<T>) -> bool {
        !(self.end <= other.start || other.end <= self.start)
    }

    fn transform_from(self: Self, other: &Range<T>) -> (Vec<Range<T>>, Vec<Range<T>>) {
        let mut v = Vec::new();
        let mut rem = Vec::new();
        if self.transform != T::zero() {
            panic!("Mapping range invoked tranform_from");
        }
        if !self.intersects(other) {
            return (v, vec![self]);
        }
        // find intersection
        let mut intersection: Range<T> = Range {
            start: T::zero(),
            end: T::zero(),
            transform: T::zero(),
        };
        if self.start < other.start {
            intersection.start = other.start;
            rem.push(Range{
                start: self.start,
                end: other.start - T::one(),
                transform: T::zero(),
            });
        } else {
            intersection.start = self.start;
        }
        if self.end > other.end {
            intersection.end = other.end;
            rem.push(Range{
                start: other.end + T::one(),
                end: self.end,
                transform: T::zero(),
            });
        } else {
            intersection.end = self.end;
        }
        intersection.start = intersection.start + other.transform;
        intersection.end = intersection.end + other.transform;
        v.push(intersection);
        (v,rem)
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

    fn transform_range(self: &Self, range: Range<i64>) -> Vec<Range<i64>> {
        let mut v = Vec::new();
        let mut rem = vec![range];
        let intersecting_ranges: Vec<&Range<i64>> = self.ranges.iter().filter(|r| r.intersects(&range)).collect();
        for r in &intersecting_ranges {
            let mut newrem = Vec::new();
            while !rem.is_empty() {
                let tr = rem.pop().unwrap();
                let (mut vpass, mut vrem) = tr.transform_from(r);
                v.append(&mut vpass);
                newrem.append(&mut vrem);
            }
            rem = newrem;
        }
        v.append(&mut rem);
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
            .map(|(x, y)| Range::new(x, x + y, 0))
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
    let maps: Vec<Range<i64>> = v
        .into_iter()
        .map(|((dest_start, source_start), range)| {
            Range::<i64>::new(source_start, source_start + range - 1, dest_start - source_start)
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
            //pause();
            let mut newvals = Vec::new();
            while let Some(val) = vals.pop() {
                let mut newval = seedmap.transform_range(val);
                newvals.append(&mut newval);
            }
            vals = newvals;
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
    }
    println!("Min location number: {}", min.start + min.transform);
    Ok(())
}
