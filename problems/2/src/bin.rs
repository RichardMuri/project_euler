// My libraries
use p2lib;

// crates.io libraries
use clap::Parser;

#[derive(Parser)]
struct Cli{
    // Max value in Fibonacci sequence to calculate (inclusive)
    max: u64,
}

// Solution for projecteuler.net/problem=2
fn main() {
    let args = Cli::parse();
    let result = p2lib::fib_sum(args.max);
    println!("{result}")
}