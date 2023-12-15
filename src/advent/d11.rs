use std::collections::HashMap;

use super::util;

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = util::parse_in_lines(&path)?.peekable();

    let size = lines.peek().ok_or("Bad input file")?.len();

    let mut grid: Vec<Vec<usize>> = Vec::new();
    for _ in 0..size {
        grid.push(vec![0; size]);
    }

    let mut next_planet = 1;

    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    grid[i][j] = next_planet;
                    next_planet += 1;
                },
                _ => {},
            }
            
        }
    }

    let mut expand_cols = Vec::new();
    let mut expand_rows = Vec::new();
    for i in 0..size {
        let mut rowsize = 0;
        let mut colsize = 0;
        for j in 0..size {
            rowsize += grid[i][j];
            colsize += grid[j][i];
        }
        if rowsize == 0 {
            expand_rows.push(i);
        }
        if colsize == 0 {
            expand_cols.push(i);
        }
    }

    for row in expand_rows.iter().rev() {
        grid.splice(row..row, [vec![0; size]]);
    }
    for col in expand_cols.iter().rev() {
        for i in 0..(size + expand_rows.len()) {
            grid[i].splice(col..col, [0]);
        }
    }

    let planets = next_planet - 1;

    let mut hm: HashMap<usize, (usize,usize)> = HashMap::new();

    for (j, row) in grid.iter().enumerate() {
        for (i, x) in row.iter().enumerate() {
            if *x != 0 {
                hm.insert(*x, (i, j));
            }
        }
    }

    let mut pairs = Vec::new();
    for i in 1..=planets {
        for j in i + 1..=planets {
            pairs.push((i, j));
        }
    }

    let mut sum = 0;

    for (a, b) in pairs {
        let (ax, ay) = hm.get(&a).unwrap();
        let (bx, by) = hm.get(&b).unwrap();
        let dist = ax.abs_diff(*bx) + ay.abs_diff(*by);
        sum += dist;
        println!("{} -> {}: {}", a, b, dist);
    }

    println!("Sum of distances: {}", sum);


    Ok(())
}

pub fn pt2(_path: String) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
