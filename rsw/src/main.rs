use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use num_bigint::BigInt;

#[derive(Clone, ValueEnum)]
enum Mode {
    Start,
    Resume,
}

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    mode: Mode,

    #[arg(short, long)]
    input: PathBuf,

    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() {
    //argument parsing
    let args = Args::parse();
    let mode = args.mode;
    let input = args.input;
    let output = match mode {
        Mode::Start => {
            let Some(output) = args.output else {
                eprintln!("--output is required in start mode");
                std::process::exit(1);
            };
            output
        }
        Mode::Resume => {
            if args.output.is_some() {
                eprintln!(
                    "--output is not allowed in resume mode, because input is used as output"
                );
                std::process::exit(1);
            }
            input.clone()
        }
    };

    //Loading values from input file
    let mut x = BigInt::default();
    let mut t = BigInt::default();
    let mut m = BigInt::default();

    match mode {
        Mode::Start => {
            let content = std::fs::read_to_string(&input).unwrap();
            for line in content.lines() {
                if let Some(value) = line.strip_prefix("x = ") {
                    x = value.trim().parse().unwrap();
                } else if let Some(value) = line.strip_prefix("t = ") {
                    t = value.trim().parse().unwrap();
                } else if let Some(value) = line.strip_prefix("m = ") {
                    m = value.trim().parse().unwrap();
                }
            }
        }
        Mode::Resume => {

        }
    }

    let mut rounds = BigInt::default();
    while rounds < t{
        x = &x * &x % &m;
        rounds += 1;
    }
}
