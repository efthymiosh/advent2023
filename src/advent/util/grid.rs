#[allow(dead_code)]
pub(crate) fn print_grid<T>(grid: &mut [&mut [T]], spacing: usize)
where
    T: Sized + std::fmt::Display,
{
    for row in grid.iter() {
        for item in row.iter() {
            print!("{0:>1$}", item, spacing);
        }
        println!();
    }
}

#[allow(dead_code)]
pub(crate) fn print_grid_pretty<T>(grid: &mut [&mut [T]], spacing: usize, empty: T)
where
    T: Sized + Eq + PartialEq + std::fmt::Display,
{
    for row in grid.iter() {
        for item in row.iter() {
            if *item == empty {
                print!("{0:>1$}", '.', spacing);
            } else {
                print!("{0:>1$}", item, spacing);
            }
        }
        println!();
    }
}

#[allow(dead_code)]
pub(crate) fn print_gridvec<T>(grid: &[Vec<T>], spacing: usize, dot: T)
where
    T: Sized + Eq + PartialEq + std::fmt::Display,
{
    for row in grid.iter() {
        for item in row.iter() {
            if *item == dot {
                print!("{0:>1$}", '.', spacing);
            } else {
                print!("{0:>1$}", item, spacing);
            }
        }
        println!();
    }
}

#[allow(dead_code)]
pub(crate) fn get_grid_neighbors((ni, nj): (usize, usize), size: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    if ni > 0 {
        neighbors.push((ni - 1, nj));
    }
    if nj > 0 {
        neighbors.push((ni, nj - 1));
    }
    if ni < size - 1 {
        neighbors.push((ni + 1, nj));
    }
    if nj < size - 1 {
        neighbors.push((ni, nj + 1));
    }
    neighbors
}
