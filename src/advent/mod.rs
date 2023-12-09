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
        _ => { print!("No such exercise found: {}, pt{}", exercise, part); Ok(()) }
    }
}
