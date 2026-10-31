use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use num_bigint::BigInt;
use std::fs::OpenOptions;
use std::io::Write;
use std::fs::File;

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

fn field(line: &str, name: &str) -> BigInt {
    let value = line
        .split("; ")
        .find_map(|part| part.strip_prefix(name)?.strip_prefix(" = "))
        .unwrap_or_else(|| panic!("missing {name} in line: {line}"));

    value
        .parse::<BigInt>()
        .unwrap_or_else(|e| panic!("invalid {name} in line: {line}: {e}"))
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
            if input == output {
                eprintln!("--input and --output must be different paths in start mode");
                std::process::exit(1);
            }
            let content = std::fs::read_to_string(&input).unwrap();
            for name in ["x = ", "t = ", "m = "] {
                if !content.lines().any(|line| line.starts_with(name)) {
                    eprintln!("input file is missing a line starting with '{name}'");
                    std::process::exit(1);
                }
            }
            for line in content.lines() {
                if let Some(value) = line.strip_prefix("x = ") {
                    x = value.trim().parse().unwrap();
                } else if let Some(value) = line.strip_prefix("t = ") {
                    t = value.trim().parse().unwrap();
                } else if let Some(value) = line.strip_prefix("m = ") {
                    m = value.trim().parse().unwrap();
                }
            }
            println!("Creating File for output");
            let mut file = File::create(&output).expect("failed to create output file");
            writeln!(file, "x = {x}; t = {t}; m = {m}").expect("failed to write to output file");
        }
        Mode::Resume => {
            let contents = std::fs::read_to_string(&input).expect("failed to read input file");
            let mut lines = contents.lines().filter(|line| !line.trim().is_empty());
            let first = lines.next().expect("input file is empty");
            x = field(first, "x");
            t = field(first, "t");
            m = field(first, "m");
            if let Some(last) = lines.last() {
                    x = field(last, "x");

                    if last.starts_with("Finished;") {
                        t = BigInt::default();
                    } else {
                        t = field(last, "remaining_t");
                    }
                }
        }
    }

    let mut rounds = BigInt::default();
    while rounds < t{
        x = &x * &x % &m;
        rounds += 1;
        if &rounds % 100_000_000 == BigInt::from(0) {
            println!("{rounds}/{t} iterations");
            let remaining_t = &t - &rounds;
            let mut file = OpenOptions::new()
                                        .append(true)
                                        .open(&output)
                                        .expect("failed to open output file");
            writeln!(file, "x = {x}; remaining_t = {remaining_t}").expect("failed to write to output file");
        }
    }
    let mut file = OpenOptions::new()
                                    .append(true)
                                    .open(&output)
                                    .expect("failed to open output file");
    writeln!(file, "Finished; x = {x}").expect("failed to write to output file");
    println!("Result: {x}");
}