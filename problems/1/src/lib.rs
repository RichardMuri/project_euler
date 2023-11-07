#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_sum_multiples()
    {
        // example case
        let max = 10;
        let result = 23;
        assert_eq!(sum_multiples(max), result);

        // case with an intersection
        let max = 16;
        let result = 60;
        assert_eq!(sum_multiples(max), result);
    }

}

pub fn sum_multiples(max: u64) -> u64
{
    let factora = 3;
    let factorb = 5;
    let x: u64 = (3..max).step_by(factora).into_iter().sum();
    let y: u64 = (5..max).step_by(factorb).into_iter().sum();
    let z: u64 = (15..max).step_by(factora * factorb).into_iter().sum();
    x+y-z
}

pub fn test()
{
    println!("Test")
}