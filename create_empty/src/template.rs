
pub fn bin_template(problem_number: u64) -> String{
    let contents = format!("// My libraries
use p{problem_number}lib;

// crates.io libraries
use clap::Parser;

#[derive(Parser)]
struct Cli{{
}}

// Solution for projecteuler.net/problem={problem_number}
fn main() {{
    let args = Cli::parse();
}}"
);
    contents 
}

pub fn lib_template(problem_number: u64) -> String{
    let contents = format!("#[cfg(test)]
mod tests{{
    use super::*;

    #[test]
    fn test_example(){{
        let expected = 0;
        let result = example();
        assert_eq!(result, example);
    }}

}}

// https://projecteuler.net/problem={problem_number}
pub fn example() -> i64{{
    0
}}"
);
    contents
}

pub fn cargo_template(problem_number: u64) -> String{
    let contents = format!("[package]
name = \"p{problem_number}\"
version = \"0.1.0\"
edition = \"2021\"
author = \"RichardMuri\"

[lib]
name = \"p{problem_number}lib\"
path = \"src/lib.rs\"

[[bin]]
name = \"p{problem_number}\"
path = \"src/bin.rs\"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
clap = {{version=\"4\", features=[\"derive\"]}}");
    contents
}