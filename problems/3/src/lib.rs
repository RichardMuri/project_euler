#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_example(){
        let expected = 29;
        let input = 13195;
        let result = factor_by_division(input);
        assert_eq!(result, expected);
    }

}

// https://projecteuler.net/problem=3
pub fn factor_by_division(input: u64) -> u64{

    // There is at most 1 prime factor greater than sqrt
    let max = (input as f64).sqrt().ceil() as u64;
    let mut factor = input;

    // Start by checking if 2 is a factor and remove all 2s
    while factor % 2 == 0 {
        factor /= 2;
    }

    // Corner case where input was 2^N
    if factor == 1
    { 
        return 2;
    }

    // Check if every odd number is a factor
    for i in (3..=max).step_by(2) {

        // If number is a factor, "remove it" as many times as necessary
        while factor % i == 0
        {
            factor /= i;
        }
        
        // Exhausted all factors, remainder must be prime
        if factor == 1
        {
            factor = i;
            break;
        }

    }
    factor
}