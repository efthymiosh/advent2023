use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, i64, multispace1};
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::IResult;
use std::collections::HashMap;

#[derive(Debug)]
struct SeedMap {
    next: String,
    maps: HashMap<i64, (i64, i64)>,
}

impl SeedMap {
    fn merge(&mut self, mut other: SeedMap) {
        println!("Merging {:#?} <--- {:#?}", self, &other);
        self.next = other.next;
        other.maps.drain().for_each(|(os, (oe, or))| {
            if let Some((e, r)) = self.maps.insert(os, (oe, or)) {
                if e == oe {
                    self.maps.insert(os, (oe, r + or));
                } else if e < oe {
                    self.maps.insert(os, (e, r + or));
                    self.maps.insert(e + 1, (oe, or));
                } else { // e > oe
                    self.maps.insert(os, (oe, r + or));
                    self.maps.insert(oe + 1, (e, r));
                }
            }
        });
        // check for overlaps
        while true {
            println!("--------------------------------");
            let mut new_values: Vec<(i64, (i64, i64))> = Vec::new();
            let mut v = self.maps.keys().collect::<Vec<&i64>>();
            v.sort();
            let mut it = v.into_iter().peekable();
            while let Some(key) = it.next() {
                if let Some(&next) = it.peek() {
                    let (e, r) = self.maps.get(key).unwrap();
                    println!("Testing e {} of key {} with next {}", e, key, next);
                    if next > e {
                        continue;
                    }
                    let (ne, nr) = self.maps.get(next).unwrap();
                    // need to split ranges
                    new_values.push((*key, (*next - 1, *r)));
                    if ne == e {
                        new_values.push((*next, (*ne, *r + *nr)));
                        println!("ne == e: inserting ({}, ({}, {}))", next, ne, r + nr);
                    } else if ne < e {
                        new_values.push((*next, (*ne, *r + *nr)));
                        new_values.push((*ne + 1, (*e, *r)));
                        println!("ne < e: inserting ({}, ({}, {}))", next, ne, r + nr);
                        println!("ne < e: inserting ({}, ({}, {}))", ne + 1, e, r);
                    } else if ne > e {
                        new_values.push((*next, (*e, *r + *nr)));
                        new_values.push((*e + 1, (*ne, *nr)));
                        println!("ne > e: inserting ({}, ({}, {}))", next, e, r + nr);
                        println!("ne > e: inserting ({}, ({}, {}))", e + 1, ne, nr);
                    }
                }
            }
            if new_values.len() == 0 {
                break;
            }
            println!("Before {:?}", self);
            new_values.iter().for_each(|(s, (e, r))| {self.maps.insert(*s, (*e, *r));});
            println!("After {:?}", self);
        }
        println!("Merged {:?}", self);
    }
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<i64>> {
    let (rem, (_seeds, list)) =
        separated_pair(tag("seeds"), tag(": "), separated_list0(tag(" "), i64))(input)?;
    Ok((rem, list))
}

fn parse_seeds_pt2(input: &str) -> IResult<&str, Vec<(i64, i64)>> {
    let (rem, (_seeds, list)) = separated_pair(
        tag("seeds"),
        tag(": "),
        separated_list0(tag(" "), separated_pair(i64, tag(" "), i64)),
    )(input)?;
    Ok((rem, list.into_iter().collect()))
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
        .map(|((map, rs), rd)| (rs, (rs + rd, map - rs)))
        .collect();
    Ok((
        rem,
        (
            mapin.to_owned(),
            SeedMap {
                next: next.to_owned(),
                maps,
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
    let seedmap = seedmaps.remove("seed").unwrap();
    let mut applymap = seedmap;
    while !seedmaps.is_empty() {
        let seedmap = seedmaps.remove(&applymap.next).unwrap();
        applymap.merge(seedmap);
    }
    let mut min = i64::max_value();
    for seed in seeds {
        let mut val = seed;
        for (rs, (re, diff)) in &applymap.maps {
            if rs <= &val && re >= &val {
                val += diff;
                break;
            }
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

    let (rem, seeds) = parse_seeds_pt2(&input).unwrap();
    println!("seeds len {}", seeds.len());
    let mut remainder = rem;
    while !remainder.is_empty() {
        let (rem, (id, seedmap)) = parse_seedmap(remainder).unwrap();
        remainder = rem;
        seedmaps.insert(id, seedmap);
    }
    let mut vl: Vec<HashMap<i64, (i64, i64)>> = Vec::new();
    let seedmap = seedmaps.remove("seed").unwrap();
    let mut next = seedmap.next;
    vl.push(seedmap.maps);
    while !seedmaps.is_empty() {
        let seedmap = seedmaps.remove(&next).unwrap();
        next = seedmap.next;
        vl.push(seedmap.maps);
    }
    let min = i64::max_value();

    println!("Min location number: {}", min);
    Ok(())
}
