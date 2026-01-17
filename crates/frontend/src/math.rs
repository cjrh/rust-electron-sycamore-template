/// Adds two integers together.
///
/// This is a pure Rust function that can be used both in WASM
/// and tested natively.
///
/// # Examples
///
/// ```
/// use frontend::math::add;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiplies two integers together.
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative_numbers() {
        assert_eq!(add(-2, -3), -5);
    }

    #[test]
    fn test_add_mixed_numbers() {
        assert_eq!(add(-2, 5), 3);
    }

    #[test]
    fn test_add_zero() {
        assert_eq!(add(0, 5), 5);
        assert_eq!(add(5, 0), 5);
    }

    #[test]
    fn test_multiply_positive_numbers() {
        assert_eq!(multiply(2, 3), 6);
    }

    #[test]
    fn test_multiply_by_zero() {
        assert_eq!(multiply(5, 0), 0);
    }

    #[test]
    fn test_multiply_negative_numbers() {
        assert_eq!(multiply(-2, -3), 6);
    }
}
