use std::fs::File;
use std::{io, io::prelude::*};

#[allow(dead_code)]
pub fn parse_in_lines(
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
pub fn pause() {
    io::stdin().read_exact(&mut [0]).unwrap();
}

#[allow(dead_code)]
pub fn print_grid(grid: &mut [&mut [u32]], spacing: usize) {
    for row in grid.iter() {
        for item in row.iter() {
            print!("{0:>1$}", item, spacing);
        }
        println!();
    }
}
