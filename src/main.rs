mod advent;
use advent::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// Exercise to run
    #[arg(short, long)]
    exercise: u8,

    /// Part to run
    #[arg(short, long)]
    part: u8,
}

fn main()  -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match (args.exercise, args.part) {
        (1,1) => { d01::pt1() }
        (1,2) => { d01::pt2() }
        (2,1) => { d02::pt1() }
        (2,2) => { d02::pt2() }
        (3,1) => { d03::pt1() }
        (3,2) => { d03::pt2() }
        _ => { print!("No such exercise found: {}, pt{}", args.exercise, args.part); Ok(()) }
    }
}
