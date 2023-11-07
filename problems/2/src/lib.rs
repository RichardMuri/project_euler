#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_fib_sum(){
        let max = 8;
        let expected = 10;
        let result = fib_sum(max);
        assert_eq!(result, expected);
    }

}

// https://projecteuler.net/problem=2
pub fn fib_sum(max: u64) -> u64{
    // First two numbers of Fibonacci sequence
    let mut previous = 1;
    let mut current = 2;
    let mut result = 0;

    while current <= max {
        // Only add even numbers
        match current % 2 == 0 {
            true => result += current,
            false => (),
        }

        // Calculate next term of sequence. Save a register with an extra subtract
        current += previous;
        previous = current - previous;
    }

    result
}