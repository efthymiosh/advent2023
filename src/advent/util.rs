use std::fs::File;
use std::{io, io::prelude::*};

#[allow(dead_code)]
pub(crate) fn parse_in_lines(
    path: &str,
) -> Result<Box<dyn Iterator<Item = String>>, Box<dyn std::error::Error>> {
    let input = File::open(path)?;
    let reader = io::BufReader::new(input);

    let iter = reader
        .lines()
        .filter_map(|l| Some(l.ok()?.trim_end().to_owned()));
    Ok(Box::new(iter))
}

#[allow(dead_code)]
pub(crate) fn pause() {
    io::stdin().read_exact(&mut [0]).unwrap();
}

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
pub(crate) fn print_gridvec<T>(grid: &Vec<Vec<T>>, spacing: usize, dot: T)
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
pub(crate) fn gcd(u: i64, v: i64) -> i64 {
    // `wrapping_abs` gives a number's absolute value, unless that's 2³¹. 2³¹
    // won't fit in `i64`, so it gives -2³¹ instead.
    let mut v = v.wrapping_abs() as u64;
    if u == 0 {
        return v as i64;
    }
    let mut u = u.wrapping_abs() as u64;
    if v == 0 {
        return u as i64;
    }

    // `|` is bitwise OR. `trailing_zeros` quickly counts a binary number's
    // trailing zeros, giving its prime factorization's exponent on two.
    let gcd_exponent_on_two = (u | v).trailing_zeros();

    // `>>=` divides the left by two to the power of the right, storing that in
    // the left variable. `u` divided by its prime factorization's power of two
    // turns it odd.
    u >>= u.trailing_zeros();
    v >>= v.trailing_zeros();

    while u != v {
        if u < v {
            // Swap the variables' values with each other.
            core::mem::swap(&mut u, &mut v);
        }
        u -= v;
        u >>= u.trailing_zeros();
    }

    // `<<` multiplies the left by two to the power of the right.
    (u << gcd_exponent_on_two) as i64
}

#[allow(dead_code)]
pub(crate) fn lcm(u: i64, v: i64) -> i64 {
    if u > v {
        (u / gcd(u, v)) * v
    } else {
        (v / gcd(v, u)) * u
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

#[allow(dead_code)]
pub (crate) fn polygon_area(vertices: &[(u64, u64)]) -> f64 {
    let n = vertices.len();
    let mut sum = 0.0;

    for cur in 0..n {
        let next = (cur + 1) % n;
        sum += (vertices[cur].0 as f64 * vertices[next].1 as f64) - (vertices[next].0 as f64 * vertices[cur].1 as f64);
    }

    0.5 * sum.abs()
}

#[allow(dead_code)]
pub (crate) fn picks_theorem(area: f64, vertices: &[(u64, u64)]) -> u64 {
    let boundary_points = vertices.len() as u64;
    let interior_points = area - (boundary_points / 2) as f64 + 1.0;

    interior_points as u64
}
