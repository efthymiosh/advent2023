use std::collections::HashMap;

use super::util;

enum Direction {
    North,
    South,
    East,
    West,
}

fn slide(grid: &mut [&mut [char]], direction: Direction) {
    match direction {
        Direction::North => {
            for j in 0..grid.len() {
                for i in 0..grid.len() {
                    if grid[i][j] != '.' {
                        continue;
                    }
                    let mut nextpos = None;
                    for ti in i..grid.len() {
                        if grid[ti][j] != '.' {
                            nextpos = Some(ti);
                            break;
                        }
                    }
                    if let Some(pos) = nextpos {
                        if grid[pos][j] == 'O' {
                            grid[i][j] = 'O';
                            grid[pos][j] = '.';
                        }
                    }
                }
            }
        },
        Direction::West => {
            for i in 0..grid.len() {
                for j in 0..grid.len() {
                    if grid[i][j] != '.' {
                        continue;
                    }
                    let mut nextpos = None;
                    for tj in j..grid.len() {
                        if grid[i][tj] != '.' {
                            nextpos = Some(tj);
                            break;
                        }
                    }
                    if let Some(pos) = nextpos {
                        if grid[i][pos] == 'O' {
                            grid[i][j] = 'O';
                            grid[i][pos] = '.';
                        }
                    }
                }
            }
        }
        Direction::South => {
            for j in (0..grid.len()).rev() {
                for i in (0..grid.len()).rev() {
                    if grid[i][j] != '.' {
                        continue;
                    }
                    let mut nextpos = None;
                    for ti in (0..i).rev() {
                        if grid[ti][j] != '.' {
                            nextpos = Some(ti);
                            break;
                        }
                    }
                    if let Some(pos) = nextpos {
                        if grid[pos][j] == 'O' {
                            grid[i][j] = 'O';
                            grid[pos][j] = '.';
                        }
                    }
                }
            }
        }
        Direction::East => {
            for i in (0..grid.len()).rev() {
                for j in (0..grid.len()).rev() {
                    if grid[i][j] != '.' {
                        continue;
                    }
                    let mut nextpos = None;
                    for tj in (0..j).rev() {
                        if grid[i][tj] != '.' {
                            nextpos = Some(tj);
                            break;
                        }
                    }
                    if let Some(pos) = nextpos {
                        if grid[i][pos] == 'O' {
                            grid[i][j] = 'O';
                            grid[i][pos] = '.';
                        }
                    }
                }
            }
        }
    }
}

fn calc_load(grid: &[&mut [char]]) -> usize {
    let mut load = 0;
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            if grid[i][j] == 'O' {
                load += grid.len() - i;
            }
        }
    }
    load
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = util::parse_in_lines(&path)?.peekable();

    let size = lines.peek().ok_or("Bad input file")?.len();

    let mut grid_raw = vec!['.'; size * size];
    let mut grid: Vec<&mut [char]> = grid_raw.as_mut_slice().chunks_mut(size).collect();

    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[i][j] = c;
        }
    }
    util::grid::print_grid(&mut grid, 2);
    slide(&mut grid, Direction::North);
    println!("Afterwards:");
    util::grid::print_grid(&mut grid, 2);
    let load = calc_load(&grid);
    println!("Total load {}", load);
    Ok(())
}

fn hash(v: &[&mut [char]]) -> String {
    v.iter().map(|e| e.iter().collect::<String>()).collect()
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = util::parse_in_lines(&path)?.peekable();

    let size = lines.peek().ok_or("Bad input file")?.len();

    let mut grid_raw = vec!['.'; size * size];
    let mut grid: Vec<&mut [char]> = grid_raw.as_mut_slice().chunks_mut(size).collect();

    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[i][j] = c;
        }
    }

    util::grid::print_grid(&mut grid, 2);
    let mut hm: HashMap<String, usize> = HashMap::new();
    let mut i = 0;
    while hm.get(&hash(&grid)).is_none() {
        hm.insert(hash(&grid), i);
        slide(&mut grid, Direction::North);
        slide(&mut grid, Direction::West);
        slide(&mut grid, Direction::South);
        slide(&mut grid, Direction::East);
        i += 1;
        util::grid::print_grid(&mut grid, 2);
        println!("{} Total load {}", i, calc_load(&grid));
    }
    let prev = hm.get(&hash(&grid)).unwrap();
    let moves = (1_000_000_000 - i) % (i - prev);
    for _ in 0..moves {
        slide(&mut grid, Direction::North);
        slide(&mut grid, Direction::West);
        slide(&mut grid, Direction::South);
        slide(&mut grid, Direction::East);
    }
    println!("Total load {}", calc_load(&grid));
    Ok(())
}
