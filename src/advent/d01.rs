use std::collections::HashMap;
use super::util;

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let lines = util::parse_in_lines(&path)?;
    let mut numbers = Vec::new();
    for line in lines {
        let string: String = line.parse()?;
        let mut first: Option<u32> = Option::None;
        let mut last: Option<u32> = Option::None;
        string.chars().for_each(|c| {
            if !c.is_digit(10) {
                return;
            }
            last = c.to_digit(10);
            if first == Option::None {
                first = last;
            }
        });
        if let (Some(f),Some(l)) = (first,last) {
            println!("line: {} -- {}{}", line, f, l);
            numbers.push(f * 10 + l);
        }
    }
    println!("{}", numbers.iter().sum::<u32>());
    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let lines = util::parse_in_lines(&path)?;
    let mut numbers = Vec::new();
    let numstrings: HashMap<&str, u32> = HashMap::from([
        ("one",1), ("two",2), ("three",3), ("four",4), ("five",5), ("six",6), ("seven",7), ("eight",8), ("nine",9),
        ("1",1), ("2",2), ("3",3), ("4",4), ("5",5), ("6",6), ("7",7), ("8",8), ("9",9),
    ]);
    for line in lines {
        let string: String = line.parse()?;
        let mut first: Option<u32> = Option::None;
        let mut last: Option<u32> = Option::None;
        for i in 0..string.len() {
            let t = string.get(i..).unwrap();
            numstrings.iter().for_each(|(&s,&i)| {
                if t.starts_with(s) {
                    last = Some(i);
                    if first == Option::None {
                        first = last;
                    }
                }
            });
        }
        if let (Some(f),Some(l)) = (first,last) {
            println!("line: {} -- {}{}", line, f, l);
            numbers.push(f * 10 + l);
        }
    }
    println!("{}", numbers.iter().sum::<u32>());
    Ok(())
}
