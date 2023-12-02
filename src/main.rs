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

    match args.exercise {
        1 => { d01::run(args.part) }
        _ => { print!("No such exercise found: {}, pt{}", args.exercise, args.part); Ok(()) }
    }
}
