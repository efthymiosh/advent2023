mod advent;
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

    advent::run(args.exercise, args.part, args.data)
}
