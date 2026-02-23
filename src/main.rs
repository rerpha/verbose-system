fn determine_fizzbuzz_output(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => n.to_string(),
    }
}

fn main() {
    for i in 1..=100 {
        println!("{}", determine_fizzbuzz_output(i));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizzbuzz_divisible_by_three() {
        assert_eq!(determine_fizzbuzz_output(3), "Fizz");
        assert_eq!(determine_fizzbuzz_output(6), "Fizz");
    }

    #[test]
    fn test_fizzbuzz_divisible_by_five() {
        assert_eq!(determine_fizzbuzz_output(5), "Buzz");
        assert_eq!(determine_fizzbuzz_output(10), "Buzz");
    }

    #[test]
    fn test_fizzbuzz_divisible_by_both() {
        assert_eq!(determine_fizzbuzz_output(15), "FizzBuzz");
        assert_eq!(determine_fizzbuzz_output(30), "FizzBuzz");
    }

    #[test]
    fn test_fizzbuzz_other_numbers() {
        assert_eq!(determine_fizzbuzz_output(1), "1");
        assert_eq!(determine_fizzbuzz_output(2), "2");
        assert_eq!(determine_fizzbuzz_output(4), "4");
    }
}