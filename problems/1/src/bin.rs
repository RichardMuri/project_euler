// My libraries
use p1lib;

// crates.io libraries
use clap::Parser;


// Return the sum of all multiples of 3 or 5 below N
#[derive(Parser)]
struct Cli{
    // Max value (exclusive)
    N: u64
} 

fn main() {
    let args = Cli::parse();
    println!("{}", p1lib::sum_multiples(args.N));
}
