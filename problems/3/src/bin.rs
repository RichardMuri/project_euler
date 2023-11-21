// My libraries
use p3lib;

// crates.io libraries
use clap::Parser;

#[derive(Parser)]
struct Cli{
    // Find the largest prime factor of input
    input: u64,
}

// Solution for projecteuler.net/problem=3
fn main() {
    let args = Cli::parse();
    let result = p3lib::factor_by_division(args.input);
    println!("{result}");
}