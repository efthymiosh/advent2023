use std::collections::HashMap;

use nom::bytes::complete::tag;
use nom::character::complete::none_of;
use nom::character::complete::u32;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

use super::util;

#[derive(Debug)]
struct Record {
    map: Vec<char>,
    counts: Vec<usize>,
}

impl Record {
    fn map_permutations(map: &[char], counts: &[usize], countcur: usize, memo: &mut HashMap<(usize, usize, usize), u64>) -> u64 {
        if let Some(&perms) = memo.get(&(map.len(), counts.len(), countcur)) {
            return perms;
        }
        if map.is_empty() {
            if counts.is_empty() || (counts.len() == 1 && counts[0] == countcur) {
                return 1;
            } else {
                return 0;
            }
        }
        let mut permutations = 0;
        match map[0] {
            '.' => {
                if countcur == 0 {
                    permutations = Record::map_permutations(&map[1..], counts, 0, memo);
                } else if counts[0] == countcur {
                    permutations = Record::map_permutations(&map[1..], &counts[1..], 0, memo);
                } else {
                    permutations = 0;
                }
            }
            '#' => {
                if counts.is_empty() {
                    return 0;
                }
                permutations = Record::map_permutations(&map[1..], counts, countcur + 1, memo);
            }
            '?' => {
                if !counts.is_empty() {
                    permutations = Record::map_permutations(&map[1..], counts, countcur + 1, memo);
                }
                if countcur == 0 {
                    permutations += Record::map_permutations(&map[1..], counts, 0, memo);
                } else if counts[0] == countcur {
                    permutations += Record::map_permutations(&map[1..], &counts[1..], 0, memo);
                }
            }
            _ => unreachable!("Received char other than expected"),
        }
        memo.insert((map.len(), counts.len(), countcur), permutations);
        permutations
    }

    fn permutations(&self) -> u64 {
        println!("{} {:?}", self.map.iter().collect::<String>(), self.counts);
        let brok = self.counts.clone();
        Record::map_permutations(&self.map[..], &brok[..], 0, &mut HashMap::new())
    }
}

fn parse_line(input: &str) -> IResult<&str, Record> {
    let (rem, (map, v)) = separated_pair(
        many1(none_of(" ")),
        tag(" "),
        separated_list1(tag(","), u32),
    )(input)?;
    Ok((
        rem,
        Record {
            map,
            counts: v.into_iter().map(|e| e as usize).collect(),
        },
    ))
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let lines = util::parse_in_lines(&path)?;

    let records: Vec<Record> = lines
        .into_iter()
        .map(|e| parse_line(&e).unwrap().1)
        .collect();

    let sum: u64 = records.iter().map(|r| r.permutations()).sum();

    println!("Sum of permutations: {}", sum);

    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let lines = util::parse_in_lines(&path)?;

    let records: Vec<Record> = lines
        .into_iter()
        .map(|e| parse_line(&e).unwrap().1)
        .map(|r| {
            let mut map = r.map.clone();
            let mut counts = r.counts.clone();
            for _ in 0..4 {
                map.push('?');
                map.append(&mut r.map.clone());
                counts.append(&mut r.counts.clone());
            }
            Record {
                map,
                counts,
            }
        })
        .collect();

    let sum: u64 = records.iter().map(|r| r.permutations()).sum();

    println!("Sum of permutations: {}", sum);

    Ok(())
}
