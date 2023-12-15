use super::util;

fn get_valid_neighbors(grid: &[&mut [char]], (x,y): (usize, usize)) -> Vec<(usize,usize)> {
    let mut valid_paths = Vec::new();
    println!("  {}  ", grid[x][y - 1]);
    println!("{} S {}", grid[x -1][y], grid[x + 1][y]);
    println!("  {}  ", grid[x][y + 1]);
    match grid[x - 1][y] {
        '-' | 'L' | 'F' => { valid_paths.push((x - 1, y)); },
        _ => {},
    };
    match grid[x][y - 1] {
        '|' | '7' | 'F' => { valid_paths.push((x , y - 1)); },
        _ => {},
    };
    match grid[x + 1][y] {
        '-' | '7' | 'J' => { valid_paths.push((x + 1, y)); },
        _ => {},
    };
    match grid[x][y + 1] {
        '|' | 'L' | 'J' => { valid_paths.push((x, y + 1)); },
        _ => {},
    };
    if valid_paths.len() != 2 {
        panic!("More than 2 valid neighbors, invalid data");
    }
    valid_paths
}

fn discover_weights(grid: &Vec<&mut [char]>, weights: &mut Vec<&mut [u64]>, (x, y): (usize, usize), parent_weight: u64) {
    if weights[x][y] <= parent_weight {
        return;
    }
    weights[x][y] = parent_weight + 1;
    match grid[x][y] {
        '|' => {
            discover_weights(grid, weights, (x, y - 1), parent_weight + 1);
            discover_weights(grid, weights, (x, y + 1), parent_weight + 1);
        }
        '-' => {
            discover_weights(grid, weights, (x + 1, y), parent_weight + 1);
            discover_weights(grid, weights, (x - 1, y), parent_weight + 1);
        }
        'L' => {
            discover_weights(grid, weights, (x + 1, y), parent_weight + 1);
            discover_weights(grid, weights, (x, y - 1), parent_weight + 1);
        }
        'J' => {
            discover_weights(grid, weights, (x - 1, y), parent_weight + 1);
            discover_weights(grid, weights, (x, y - 1), parent_weight + 1);
        }
        '7' => {
            discover_weights(grid, weights, (x - 1, y), parent_weight + 1);
            discover_weights(grid, weights, (x, y + 1), parent_weight + 1);
        }
        'F' => {
            discover_weights(grid, weights, (x + 1, y), parent_weight + 1);
            discover_weights(grid, weights, (x, y + 1), parent_weight + 1);
        }
        _ => {
            panic!("Reached unexpected {}", grid[x][y]);
        }
    }
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = util::parse_in_lines(&path)?.peekable();

    let size = lines.peek().ok_or("Bad input file")?.len();

    let mut grid_raw = vec![' '; size * size];
    let mut grid: Vec<&mut [char]> = grid_raw.as_mut_slice().chunks_mut(size).collect();
    let mut weights_raw = vec![u64::max_value(); size * size];
    let mut weights: Vec<&mut [u64]> = weights_raw.as_mut_slice().chunks_mut(size).collect();

    let (mut startx, mut starty) = (0,0);
    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[j][i] = c;
            if c == 'S' {
                (startx, starty) = (j, i);
            }
        }
    }

    util::print_grid(grid.as_mut_slice(), 1);

    weights[startx][starty] = 0;
    let v = get_valid_neighbors(&grid, (startx, starty));

    for (x,y) in v {
        discover_weights(&grid, &mut weights, (x, y), 0);
    }
    weights_raw.iter_mut().for_each(|e| if *e == u64::max_value() { *e = 0 });

    let mut weights: Vec<&mut [u64]> = weights_raw.as_mut_slice().chunks_mut(size).collect();
    util::print_grid(weights.as_mut_slice(), 1);
    if let Some(max) = weights_raw.iter().max() {
        println!("Max value: {}", max);
    }

    Ok(())
}

pub fn pt2(_path: String) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
