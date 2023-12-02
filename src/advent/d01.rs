use super::util;

pub fn run(pt: u8) -> Result<(), Box<dyn std::error::Error>> {
    if pt == 1 {
        pt1()
    } else {
        pt2()
    }
}

fn pt1() -> Result<(), Box<dyn std::error::Error>> {
    let lines = util::parse_in_lines("./data/ex01pt1.txt")?;
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

fn pt2() -> Result<(), Box<dyn std::error::Error>> {
    todo!()
}
