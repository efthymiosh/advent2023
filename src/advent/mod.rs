mod util;
mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d20;

pub fn run (exercise: u8, part: u8, file: String) -> Result<(), Box<dyn std::error::Error>> {
    match (exercise, part) {
        (1,1) => { d01::pt1(file) }
        (1,2) => { d01::pt2(file) }
        (2,1) => { d02::pt1(file) }
        (2,2) => { d02::pt2(file) }
        (3,1) => { d03::pt1(file) }
        (3,2) => { d03::pt2(file) }
        (4,1) => { d04::pt1(file) }
        (4,2) => { d04::pt2(file) }
        (5,1) => { d05::pt1(file) }
        (5,2) => { d05::pt2(file) }
        (6,1) => { d06::pt1(file) }
        (6,2) => { d06::pt2(file) }
        (7,1) => { d07::pt1(file) }
        (7,2) => { d07::pt2(file) }
        (8,1) => { d08::pt1(file) }
        (8,2) => { d08::pt2(file) }
        (9,1) => { d09::pt1(file) }
        (9,2) => { d09::pt2(file) }
        (10,1) => { d10::pt1(file) }
        (10,2) => { d10::pt2(file) }
        (11,1) => { d11::pt1(file) }
        (11,2) => { d11::pt2(file) }
        (12,1) => { d12::pt1(file) }
        (12,2) => { d12::pt2(file) }
        (13,1) => { d13::pt1(file) }
        (13,2) => { d13::pt2(file) }
        (14,1) => { d14::pt1(file) }
        (14,2) => { d14::pt2(file) }
        (15,1) => { d15::pt1(file) }
        (15,2) => { d15::pt2(file) }
        (16,1) => { d16::pt1(file) }
        (16,2) => { d16::pt2(file) }
        (17,1) => { d17::pt1(file) }
        (17,2) => { d17::pt2(file) }
        (18,1) => { d18::pt1(file) }
        (18,2) => { d18::pt2(file) }
        (19,1) => { d19::pt1(file) }
        (19,2) => { d19::pt2(file) }
        (20,1) => { d20::pt1(file) }
        (20,2) => { d20::pt2(file) }
        _ => { print!("No such exercise found: {}, pt{}", exercise, part); Ok(()) }
    }
}
