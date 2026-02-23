/// FizzBuzz Application - Verbose Implementation
///
/// This is a comprehensive, well-documented FizzBuzz implementation in Rust.
/// The FizzBuzz problem is a classic programming challenge that demonstrates:
/// - Looping and iteration
/// - Conditional logic and pattern matching
/// - String manipulation
/// - Modulo operator for divisibility checking
///
/// Rules:
/// - Print numbers from 1 to 100
/// - For multiples of 3, print "Fizz" instead of the number
/// - For multiples of 5, print "Buzz" instead of the number
/// - For multiples of both 3 and 5, print "FizzBuzz" instead of the number

/// The main entry point of the application
fn main() {
    println!("=== FizzBuzz Application ===");
    println!("Starting FizzBuzz generation from 1 to 100...\n");

    // Call the fizzbuzz function to perform the main operation
    fizzbuzz();

    println!("\n=== FizzBuzz Application Complete ===");
}

/// Executes the FizzBuzz algorithm
///
/// This function iterates through numbers 1 to 100 (inclusive) and applies
/// the FizzBuzz rules to each number. It demonstrates the use of:
/// - Range iteration (1..=100)
/// - Pattern matching and conditional logic
/// - Modulo operator (%) for divisibility testing
fn fizzbuzz() {
    // Iterate through each number from 1 to 100 (inclusive)
    // The range 1..=100 means: start at 1, go up to and including 100
    for number in 1..=100 {
        // Apply the FizzBuzz logic to determine what to print
        let output = determine_fizzbuzz_output(number);

        // Print the result to the console
        println!("{}", output);
    }
}

/// Determines the appropriate output for a given number according to FizzBuzz rules
///
/// # Arguments
/// * `number` - An integer value to evaluate
///
/// # Returns
/// A String containing either:
/// - "FizzBuzz" if the number is divisible by both 3 and 5
/// - "Fizz" if the number is divisible by 3
/// - "Buzz" if the number is divisible by 5
/// - The number itself (as a string) if none of the above conditions are met
fn determine_fizzbuzz_output(number: i32) -> String {
    // Check if the number is divisible by both 3 and 5
    // The modulo operator (%) returns the remainder after division
    // If remainder is 0, the number is evenly divisible
    if number % 3 == 0 && number % 5 == 0 {
        // Both divisors apply - return "FizzBuzz"
        // Note: This condition must be checked FIRST because it's the most specific
        String::from("FizzBuzz")
    }
    // Check if the number is divisible by 3 only
    else if number % 3 == 0 {
        // Divisible by 3 but not by 5
        String::from("Fizz")
    }
    // Check if the number is divisible by 5 only
    else if number % 5 == 0 {
        // Divisible by 5 but not by 3
        String::from("Buzz")
    }
    // If none of the above conditions match
    else {
        // Return the number itself as a string
        number.to_string()
    }
}

/// Alternative implementation using pattern matching (more idiomatic Rust)
///
/// This version demonstrates Rust's powerful pattern matching capabilities
/// and can be used instead of the if-else chain above for a more elegant solution
#[allow(dead_code)]
fn determine_fizzbuzz_output_pattern_match(number: i32) -> String {
    // Use pattern matching to evaluate the conditions
    // This is a more functional programming approach
    match (number % 3 == 0, number % 5 == 0) {
        // Pattern: (true, true) - divisible by both 3 and 5
        (true, true) => String::from("FizzBuzz"),
        // Pattern: (true, false) - divisible by 3 only
        (true, false) => String::from("Fizz"),
        // Pattern: (false, true) - divisible by 5 only
        (false, true) => String::from("Buzz"),
        // Pattern: (false, false) - divisible by neither
        (false, false) => number.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test case: Number divisible by both 3 and 5 (15)
    #[test]
    fn test_fizzbuzz_divisible_by_both() {
        assert_eq!(determine_fizzbuzz_output(15), "FizzBuzz");
        assert_eq!(determine_fizzbuzz_output(30), "FizzBuzz");
        assert_eq!(determine_fizzbuzz_output(60), "FizzBuzz");
    }

    /// Test case: Number divisible by 3 only
    #[test]
    fn test_fizzbuzz_divisible_by_three() {
        assert_eq!(determine_fizzbuzz_output(3), "Fizz");
        assert_eq!(determine_fizzbuzz_output(9), "Fizz");
        assert_eq!(determine_fizzbuzz_output(12), "Fizz");
    }

    /// Test case: Number divisible by 5 only
    #[test]
    fn test_fizzbuzz_divisible_by_five() {
        assert_eq!(determine_fizzbuzz_output(5), "Buzz");
        assert_eq!(determine_fizzbuzz_output(10), "Buzz");
        assert_eq!(determine_fizzbuzz_output(20), "Buzz");
    }

    /// Test case: Number not divisible by 3 or 5
    #[test]
    fn test_fizzbuzz_neither() {
        assert_eq!(determine_fizzbuzz_output(1), "1");
        assert_eq!(determine_fizzbuzz_output(2), "2");
        assert_eq!(determine_fizzbuzz_output(7), "7");
    }
}