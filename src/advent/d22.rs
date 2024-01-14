use nom::bytes::complete::tag;
use nom::character::complete::u32;
use nom::multi::separated_list1;
use nom::{sequence::separated_pair, IResult};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};

#[allow(unused_imports)]
use super::util;

#[derive(Debug, PartialEq, Eq)]
struct Block {
    id: usize,
    start: (usize, usize, usize),
    end: (usize, usize, usize),
}

fn stack_bricks(grid: &mut [Vec<Vec<usize>>], blocks: &mut Vec<Block>) {
    blocks.sort_by(|a, b| {
        let zord = a.start.2.cmp(&b.start.2);
        if zord != Ordering::Equal {
            return zord;
        }
        let yord = a.start.1.cmp(&b.start.1);
        if yord != Ordering::Equal {
            return yord;
        }
        a.start.0.cmp(&b.start.0)
    });

    for block in blocks {
        let mut delta_empty = Vec::new();
        let z = block.start.2;
        for x in block.start.0..=block.end.0 {
            for y in block.start.1..=block.end.1 {
                let mut delta_z = 0;
                for test_z in (0..z).rev() {
                    if grid[x][y][test_z] != 0 {
                        break;
                    }
                    delta_z = z - test_z;
                }
                delta_empty.push(delta_z);
            }
        }
        let delta;
        if let Some(&x) = delta_empty.iter().min() {
            delta = x;
        } else {
            continue;
        }
        for z in block.start.2..=block.end.2 {
            for x in block.start.0..=block.end.0 {
                for y in block.start.1..=block.end.1 {
                    grid[x][y][z - delta] = grid[x][y][z];
                    grid[x][y][z] = 0;
                }
            }
        }
        block.start.2 -= delta;
        block.end.2 -= delta;
    }
}

fn parse_block(input: &str) -> IResult<&str, Block> {
    let (rem, (start, end)) = separated_pair(
        separated_list1(tag(","), u32),
        tag("~"),
        separated_list1(tag(","), u32),
    )(input)?;
    Ok((
        rem,
        Block {
            id: rem.len() + 1,
            start: (start[0] as usize, start[1] as usize, start[2] as usize),
            end: (end[0] as usize, end[1] as usize, end[2] as usize),
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Block>> {
    separated_list1(tag("\n"), parse_block)(input)
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let input: String = std::fs::read_to_string(path)?.trim().parse()?;

    let (rem, mut blocks) = parse_input(&input).unwrap();
    if !rem.is_empty() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Input not fully consumed",
        )));
    }

    let zmax = blocks
        .iter()
        .fold(0, |s, b| if b.end.2 > s { b.end.2 } else { s });
    let ymax = blocks
        .iter()
        .fold(0, |s, b| if b.end.1 > s { b.end.1 } else { s });
    let xmax = blocks
        .iter()
        .fold(0, |s, b| if b.end.0 > s { b.end.0 } else { s });

    let mut grid = vec![vec![vec![0; zmax + 2]; ymax + 1]; xmax + 1];

    for block in &blocks {
        for x in block.start.0..=block.end.0 {
            for y in block.start.1..=block.end.1 {
                for z in block.start.2..=block.end.2 {
                    grid[x][y][z] = block.id;
                }
            }
        }
    }
    stack_bricks(&mut grid, &mut blocks);

    // construct block deps map: id -> (amount blocks we stand on, if is empty above)
    let mut block_stands_on: HashMap<usize, usize> = HashMap::new();

    let mut removable = 0;

    for block in &blocks {
        let mut stands_on: HashSet<usize> = HashSet::new();
        for x in block.start.0..=block.end.0 {
            for y in block.start.1..=block.end.1 {
                let z = block.start.2;
                if z > 0 && grid[x][y][z - 1] != 0 {
                    stands_on.insert(grid[x][y][z - 1]);
                }
            }
        }
        block_stands_on.insert(block.id, stands_on.len());
    }

    for block in &blocks {
        let mut safe = true;
        for x in block.start.0..=block.end.0 {
            for y in block.start.1..=block.end.1 {
                let z = block.end.2;
                let block_above = grid[x][y][z + 1];
                if block_above != 0 && block_stands_on[&block_above] == 1 {
                    safe = false;
                }
            }
        }
        if safe {
            removable += 1;
        }
    }

    println!("Removable blocks: {}", removable);
    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let input: String = std::fs::read_to_string(path)?.trim().parse()?;

    let (rem, mut blocks) = parse_input(&input).unwrap();
    if !rem.is_empty() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Input not fully consumed",
        )));
    }

    let zmax = blocks
        .iter()
        .fold(0, |s, b| if b.end.2 > s { b.end.2 } else { s });
    let ymax = blocks
        .iter()
        .fold(0, |s, b| if b.end.1 > s { b.end.1 } else { s });
    let xmax = blocks
        .iter()
        .fold(0, |s, b| if b.end.0 > s { b.end.0 } else { s });

    let mut grid = vec![vec![vec![0; zmax + 2]; ymax + 1]; xmax + 1];

    for block in &blocks {
        for x in block.start.0..=block.end.0 {
            for y in block.start.1..=block.end.1 {
                for z in block.start.2..=block.end.2 {
                    grid[x][y][z] = block.id;
                }
            }
        }
    }
    stack_bricks(&mut grid, &mut blocks);

    // construct block deps map: id -> (amount blocks we stand on, if is empty above)
    let mut block_stands_on: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut block_stands_below: HashMap<usize, HashSet<usize>> = HashMap::new();

    for block in &blocks {
        let mut stands_on: HashSet<usize> = HashSet::new();
        for x in block.start.0..=block.end.0 {
            for y in block.start.1..=block.end.1 {
                let z = block.start.2;
                if z > 0 && grid[x][y][z - 1] != 0 {
                    stands_on.insert(grid[x][y][z - 1]);
                }
            }
        }
        block_stands_on.insert(block.id, stands_on);
    }

    for block in &blocks {
        let blockdeps = block_stands_on.get(&block.id).unwrap();
        for dep in blockdeps {
            if let Some(hs) = block_stands_below.get_mut(dep) {
                hs.insert(block.id);
            } else {
                let mut hs = HashSet::new();
                hs.insert(block.id);
                block_stands_below.insert(*dep, hs);
            }
        }
    }

    let mut total_fallout = 0;

    for block in blocks.iter().rev() {
        let mut blocks_dropped = HashSet::new();
        // adding self, must be subtracted when calculating fallout as it does not count as fallout
        blocks_dropped.insert(block.id);
        let mut queue = VecDeque::new();
        queue.push_back(block.id);
        while let Some(block_id) = queue.pop_front() {
            if let Some(blocksabove) = block_stands_below.get(&block_id) {
                for dep in blocksabove {
                    queue.push_back(*dep);
                    let dep_stands_on = &block_stands_on[&dep];
                    if dep_stands_on.is_subset(&blocks_dropped) {
                        blocks_dropped.insert(*dep);
                    }
                }
            }
        }
        total_fallout += blocks_dropped.len() - 1;
    }

    println!("Total fallout from blocks: {}", total_fallout);
    Ok(())
}
