// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_clam() {
        let result = crate::clamp(50, 49, 51);
        let expected = 50;

        assert_eq!(result, expected, "Clamp is not working correctly")
    }

    #[test]
    fn test_div() {
        let result = crate::div(50, 5);
        let expected = Some(10);

        assert_eq!(result, expected, "Div is not working correctly")
    }

    #[test]
    fn test_concat() {
        let result = crate::concat("Rounak", "Joshi");
        let expected = "RounakJoshi";

        assert_eq!(result, expected, "Concat is not working correctly")
    }
}

fn main() {}
