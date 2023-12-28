use std::fmt::Display;

use super::util;

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct LightDirections: u8 {
        const NORTH = 0b0001;
        const WEST  = 0b0100;
        const SOUTH = 0b0010;
        const EAST  = 0b1000;
    }
}

#[derive(Debug, Clone)]
struct Point {
    dirs: LightDirections,
    tile: char,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = if self.dirs.is_empty() { self.tile } else { '#' };
        write!(f, "{}", c)
    }
}

fn energize(grid: &mut Vec<&mut [Point]>, p: (isize, isize), to: LightDirections) -> u64 {
    if p.0 < 0 || p.1 < 0 || p.0 >= grid.len() as isize || p.1 >= grid.len() as isize {
        return 0;
    }
    let (i, j) = (p.0 as usize, p.1 as usize);
    let next_dirs = match grid[i][j].tile {
        '.' => to,
        '\\' => match to {
            LightDirections::NORTH => LightDirections::WEST,
            LightDirections::EAST => LightDirections::SOUTH,
            LightDirections::SOUTH => LightDirections::EAST,
            LightDirections::WEST => LightDirections::NORTH,
            _ => unreachable!("Caught from that does not match"),
        },
        '/' => match to {
            LightDirections::NORTH => LightDirections::EAST,
            LightDirections::WEST => LightDirections::SOUTH,
            LightDirections::SOUTH => LightDirections::WEST,
            LightDirections::EAST => LightDirections::NORTH,
            _ => unreachable!("Caught from that does not match"),
        },
        '|' => match to {
            LightDirections::EAST | LightDirections::WEST => {
                LightDirections::NORTH | LightDirections::SOUTH
            }
            _ => to,
        },
        '-' => match to {
            LightDirections::NORTH | LightDirections::SOUTH => {
                LightDirections::EAST | LightDirections::WEST
            }
            _ => to,
        },
        _ => {
            unreachable!("No grid match")
        }
    };
    if grid[i][j].dirs.contains(next_dirs) {
        // no change here, already accounted for
        0
    } else {
        let mut ret = if grid[i][j].dirs == LightDirections::empty() {
            1
        } else {
            0
        };
        grid[i][j].dirs |= next_dirs;
        for light in next_dirs.iter() {
            ret += match light {
                LightDirections::NORTH => energize(grid, (p.0 - 1, p.1), light),
                LightDirections::SOUTH => energize(grid, (p.0 + 1, p.1), light),
                LightDirections::EAST => energize(grid, (p.0, p.1 + 1), light),
                LightDirections::WEST => energize(grid, (p.0, p.1 - 1), light),
                _ => {
                    unreachable!("No grid match")
                }
            };
        }
        ret
    }
}
pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = util::parse_in_lines(&path)?.peekable();
    let size = lines.peek().ok_or("Bad input file")?.len();
    let mut grid_raw = vec![
        Point {
            dirs: LightDirections::empty(),
            tile: '.',
        };
        size * size
    ];
    let mut grid: Vec<&mut [Point]> = grid_raw.as_mut_slice().chunks_mut(size).collect();
    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[i][j].tile = c;
        }
    }
    util::grid::print_grid(&mut grid, 2);
    println!(
        "Total energized {}",
        energize(&mut grid, (0, 0), LightDirections::EAST)
    );
    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = util::parse_in_lines(&path)?.peekable();
    let size = lines.peek().ok_or("Bad input file")?.len();
    let mut grid_raw = vec![
        Point {
            dirs: LightDirections::empty(),
            tile: '.',
        };
        size * size
    ];
    let mut grid: Vec<&mut [Point]> = grid_raw.as_mut_slice().chunks_mut(size).collect();
    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[i][j].tile = c;
        }
    }
    util::grid::print_grid(&mut grid, 2);

    let mut max_energized = 0;
    for i in 0..size as isize {
        let mut newgrid_raw = grid_raw.clone();
        let mut grid: Vec<&mut [Point]> = newgrid_raw.as_mut_slice().chunks_mut(size).collect();

        max_energized = max_energized.max(energize(&mut grid, (i, 0), LightDirections::EAST));

        let mut newgrid_raw = grid_raw.clone();
        let mut grid: Vec<&mut [Point]> = newgrid_raw.as_mut_slice().chunks_mut(size).collect();
        max_energized = max_energized.max(energize(
            &mut grid,
            (i, size as isize - 1),
            LightDirections::WEST,
        ));

        let mut newgrid_raw = grid_raw.clone();
        let mut grid: Vec<&mut [Point]> = newgrid_raw.as_mut_slice().chunks_mut(size).collect();
        max_energized = max_energized.max(energize(&mut grid, (0, i), LightDirections::SOUTH));

        let mut newgrid_raw = grid_raw.clone();
        let mut grid: Vec<&mut [Point]> = newgrid_raw.as_mut_slice().chunks_mut(size).collect();
        max_energized = max_energized.max(energize(
            &mut grid,
            (size as isize - 1, i),
            LightDirections::NORTH,
        ));
    }
    println!("Max energized {}", max_energized);
    Ok(())
}
