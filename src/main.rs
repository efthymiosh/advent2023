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

    /// Datafile to use
    #[arg(short, long)]
    data: String,
}

fn main()  -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match (args.exercise, args.part) {
        (1,1) => { d01::pt1(args.data) }
        (1,2) => { d01::pt2(args.data) }
        (2,1) => { d02::pt1(args.data) }
        (2,2) => { d02::pt2(args.data) }
        (3,1) => { d03::pt1(args.data) }
        (3,2) => { d03::pt2(args.data) }
        (4,1) => { d04::pt1(args.data) }
        (4,2) => { d04::pt2(args.data) }
        (5,1) => { d05::pt1(args.data) }
        (5,2) => { d05::pt2(args.data) }
        (6,1) => { d06::pt1(args.data) }
        (6,2) => { d06::pt2(args.data) }
        (7,1) => { d07::pt1(args.data) }
        (7,2) => { d07::pt2(args.data) }
        _ => { print!("No such exercise found: {}, pt{}", args.exercise, args.part); Ok(()) }
    }
}
