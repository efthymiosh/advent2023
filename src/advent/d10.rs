use super::util;

fn get_valid_neighbors(grid: &[&mut [char]], (x,y): (usize, usize)) -> Vec<(usize,usize)> {
    let mut valid_paths = Vec::new();
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

fn discover_weights(grid: &Vec<&mut [char]>, weights: &mut Vec<&mut [i64]>, (x, y): (usize, usize), parent_weight: i64) {
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

fn discover_loop(grid: &Vec<&mut [char]>, mark: &mut Vec<&mut [i64]>, vertices: &mut Vec<(i64, i64)>, (x, y): (usize, usize)) {
    if mark[x][y] == 1 {
        return;
    }
    mark[x][y] = 1;
    vertices.push((x as i64, y as i64));
    match grid[x][y] {
        '|' => {
            discover_loop(grid, mark, vertices, (x, y - 1));
            discover_loop(grid, mark, vertices, (x, y + 1));
        }
        '-' => {
            discover_loop(grid, mark, vertices, (x + 1, y));
            discover_loop(grid, mark, vertices, (x - 1, y));
        }
        'L' => {
            discover_loop(grid, mark, vertices, (x + 1, y));
            discover_loop(grid, mark, vertices, (x, y - 1));
        }
        'J' => {
            discover_loop(grid, mark, vertices, (x - 1, y));
            discover_loop(grid, mark, vertices, (x, y - 1));
        }
        '7' => {
            discover_loop(grid, mark, vertices, (x - 1, y));
            discover_loop(grid, mark, vertices, (x, y + 1));
        }
        'F' => {
            discover_loop(grid, mark, vertices, (x + 1, y));
            discover_loop(grid, mark, vertices, (x, y + 1));
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
    let mut weights_raw = vec![i64::max_value(); size * size];
    let mut weights: Vec<&mut [i64]> = weights_raw.as_mut_slice().chunks_mut(size).collect();

    let (mut startx, mut starty) = (0,0);
    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[j][i] = c;
            if c == 'S' {
                (startx, starty) = (j, i);
            }
        }
    }

    util::grid::print_grid(grid.as_mut_slice(), 1);

    weights[startx][starty] = 0;
    let v = get_valid_neighbors(&grid, (startx, starty));

    for (x,y) in v {
        discover_weights(&grid, &mut weights, (x, y), 0);
    }
    weights_raw.iter_mut().for_each(|e| if *e == i64::max_value() { *e = 0 });

    let mut weights: Vec<&mut [i64]> = weights_raw.as_mut_slice().chunks_mut(size).collect();
    util::grid::print_grid(weights.as_mut_slice(), 1);
    if let Some(max) = weights_raw.iter().max() {
        println!("Max value: {}", max);
    }

    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = util::parse_in_lines(&path)?.peekable();

    let size = lines.peek().ok_or("Bad input file")?.len();

    let mut grid_raw = vec![' '; size * size];
    let mut grid: Vec<&mut [char]> = grid_raw.as_mut_slice().chunks_mut(size).collect();
    let mut mark_raw = vec![0; size * size];
    let mut mark: Vec<&mut [i64]> = mark_raw.as_mut_slice().chunks_mut(size).collect();

    let (mut startx, mut starty) = (0,0);
    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[j][i] = c;
            if c == 'S' {
                (startx, starty) = (j, i);
            }
        }
    }
    mark[startx][starty] = 1;
    let v = get_valid_neighbors(&grid, (startx, starty));
    let mut vertices: Vec<(i64, i64)> = Vec::new();
    vertices.push((startx as i64, starty as i64));
    for (x,y) in v {
        discover_loop(&grid, &mut mark, &mut vertices, (x, y));
    }

    let area = util::math::polygon_area(vertices.as_slice());
    println!("Area: {}\nPolygon Count: {}", area, vertices.len());
    let inside_tiles: u64 = util::math::picks_theorem(area, vertices.as_slice());

    println!("Inside tiles: {:?}", inside_tiles);

    Ok(())
}
