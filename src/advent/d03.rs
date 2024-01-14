use std::collections::HashSet;

use super::util;

const MARK: char = 'm';
const EMPTY: char = '.';

fn is_part(grid: &mut [&mut [char]], line: usize, mut start: usize, mut end: usize) -> bool {
    start = start.saturating_sub(1);
    if end != grid[line].len() - 1 {
        end += 1;
    }
    let mut lstart = 0;
    let mut lend = line + 1;
    if line > 0 {
        lstart = line - 1;
    }
    if lend >= grid.len() {
        lend = grid.len() - 1;
    }
    for i in lstart..=lend {
        for j in start..=end {
            if grid[i][j] == MARK {
                return true;
            }
        }
    }
    false
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = util::parse_in_lines(&path)?.peekable();
    let mut sum = 0;

    let size = lines.peek().ok_or("Bad input file")?.len();

    let mut grid_raw = vec![EMPTY; size * size];
    let mut grid_base: Vec<&mut [char]> = grid_raw.as_mut_slice().chunks_mut(size).collect();
    let grid = grid_base.as_mut_slice();

    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[i][j] = match c {
                '.' => EMPTY,
                x if x.is_ascii_digit() => x,
                _ => MARK,
            }
        }
    }

    for i in 0..size {
        let mut j = 0;
        while j < size {
            let jstart = j;
            if !grid[i][jstart].is_ascii_digit() {
                j += 1;
                continue;
            }
            let mut end = size;
            for k in jstart..size {
                if !grid[i][k].is_ascii_digit() {
                    end = k;
                    break;
                }
                j += 1;
            }
            if !is_part(grid, i, jstart, end - 1) {
                j += 1;
                continue;
            }
            let slice = &grid[i][jstart..end];
            let part = slice.iter().collect::<String>().parse::<u32>().unwrap();
            sum += part;
            j += 1;
        }
    }
    println!("Sum of parts: {}", sum);
    Ok(())
}

fn find_gears(grid: &mut [&mut [char]], i: usize, j: usize) -> Option<u32> {
    let mut positions = HashSet::<(usize, usize)>::new();
    for is in i - 1 ..= i + 1 {
        for js in j - 1 ..= j + 1 {
            if !grid[is][js].is_ascii_digit() {
                continue;
            }
            let mut numstart = js;
            for x in (0..=js).rev() {
                if !grid[is][x].is_ascii_digit() {
                    break;
                }
                numstart = x;
            }
            positions.insert((is,numstart));
        }
    }
    if positions.len() != 2 {
        return None
    }
    let mut gear = 1;
    for (x,y) in positions {
        gear *= grid[x][y..].iter().take_while(|c| c.is_ascii_digit()).collect::<String>().parse::<u32>().unwrap();
    }
    Some(gear)
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = util::parse_in_lines(&path)?.peekable();
    let mut sum = 0;

    let size = lines.peek().ok_or("Bad input file")?.len();

    let mut grid_raw = vec![EMPTY; (size + 2) * (size + 2)];
    let mut grid_base: Vec<&mut [char]> = grid_raw.as_mut_slice().chunks_mut(size + 2).collect();
    let grid = grid_base.as_mut_slice();

    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[i+1][j+1] = match c {
                x if x.is_ascii_digit() => x,
                '*' => MARK,
                _ => EMPTY,
            }
        }
    }
    util::grid::print_grid(grid, 1);
    for i in 1..=size {
        for j in 1..=size {
            if grid[i][j] != MARK {
                continue;
            }
            if let Some(gear) = find_gears(grid, i, j) {
                sum += gear;
            }
        }
    }
    println!("Sum of gears: {}", sum);
    Ok(())
}
