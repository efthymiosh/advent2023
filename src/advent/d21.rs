use std::collections::HashSet;

use super::util;

fn take_step(grid: &Vec<&mut [char]>, stepcount: usize, pos: (isize, isize), endpos: &mut Vec<(usize, usize)>, visited: &mut HashSet<(isize, isize, usize)>, maxsteps: usize) {
    let ipos = (
        if pos.0 > 0 {
            pos.0 % grid.len() as isize
        } else {
            ((pos.0 % grid.len() as isize) + grid.len() as isize) % grid.len() as isize
        },
        if pos.1 > 0 {
            pos.1 % grid.len() as isize
        } else {
            ((pos.1 % grid.len() as isize) + grid.len() as isize) % grid.len() as isize
        },
    );
    if grid[ipos.0 as usize][ipos.1 as usize] == '#' {
        return;
    }
    if visited.contains(&(pos.0, pos.1, stepcount)) {
        return;
    }
    if visited.contains(&(pos.0, pos.1, stepcount)) {
        return;
    }
    if stepcount == maxsteps {
        endpos.push((pos.0 as usize, pos.1 as usize)); return;
    }
    visited.insert((pos.0, pos.1, stepcount));
    take_step(grid, stepcount + 1, (pos.0 - 1, pos.1), endpos, visited, maxsteps);
    take_step(grid, stepcount + 1, (pos.0 + 1, pos.1), endpos, visited, maxsteps);
    take_step(grid, stepcount + 1, (pos.0, pos.1 - 1), endpos, visited, maxsteps);
    take_step(grid, stepcount + 1, (pos.0, pos.1 + 1), endpos, visited, maxsteps);
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = util::parse_in_lines(&path)?.peekable();

    let size = lines.peek().ok_or("Bad input file")?.len();

    let mut grid_raw = vec![' '; size * size];
    let mut grid: Vec<&mut [char]> = grid_raw.as_mut_slice().chunks_mut(size).collect();

    let (mut startx, mut starty) = (0,0);
    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[j][i] = c;
            if c == 'S' {
                (startx, starty) = (j as isize, i as isize);
            }
        }
    }

    util::grid::print_grid(grid.as_mut_slice(), 1);
    let mut endpos = Vec::new();
    let mut hm = HashSet::new();
    take_step(&grid, 0, (startx, starty), &mut endpos, &mut hm, 64);
    let end: HashSet<(usize, usize)> = endpos.into_iter().collect();
    println!("End positions: {:?}", end.len());

    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = util::parse_in_lines(&path)?.peekable();

    let size = lines.peek().ok_or("Bad input file")?.len();

    let mut grid_raw = vec![' '; size * size];
    let mut grid: Vec<&mut [char]> = grid_raw.as_mut_slice().chunks_mut(size).collect();

    let (mut startx, mut starty) = (0,0);
    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[j][i] = c;
            if c == 'S' {
                (startx, starty) = (j as isize, i as isize);
            }
        }
    }

    let rem = 26501365 % grid.len();

    let mut solutions = Vec::new();

    for steps in [rem, grid.len() + rem, 2 * grid.len() + rem] {
        let mut endpos = Vec::new();
        let mut visited = HashSet::new();
        take_step(&grid, 0, (startx, starty), &mut endpos, &mut visited, steps);
        let end: HashSet<(usize, usize)> = endpos.into_iter().collect();
        solutions.push(end.len());
        println!("End positions for {} steps: {:?}", steps, end.len());
    }

    let c = solutions[0];
    let a_plus_b = solutions[1] - c;
    let four_aplus_two_b = solutions[2] - c;
    let two_a = four_aplus_two_b - (2 * a_plus_b);
    let a = two_a / 2;
    let b = a_plus_b - a;

    for n in 0..3 {
        println!("f[{}]: {}", n, a * n * n + b * n + c);
    }
    let x = 26501365 / grid.len();
    println!("f[{}]: {}", x, a * x * x + b * x + c);

    Ok(())
}
