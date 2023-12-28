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
