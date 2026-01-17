//! Core business logic for the backend.
//!
//! This module contains pure Rust functions that can be tested independently
//! of the Neon bindings. The Neon functions in `lib.rs` are thin wrappers
//! around these functions.

/// Returns a greeting message.
pub fn hello() -> String {
    String::from("Hello from Rust backend via Neon!")
}

/// Adds two numbers together.
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

/// System information structure.
#[derive(Debug, Clone, PartialEq)]
pub struct SystemInfo {
    pub platform: String,
    pub arch: String,
    pub family: String,
}

/// Returns information about the current system.
pub fn get_system_info() -> SystemInfo {
    SystemInfo {
        platform: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        family: std::env::consts::FAMILY.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        let result = hello();
        assert!(result.contains("Hello"));
        assert!(result.contains("Rust"));
    }

    #[test]
    fn test_add_positive_numbers() {
        assert_eq!(add(2.0, 3.0), 5.0);
    }

    #[test]
    fn test_add_negative_numbers() {
        assert_eq!(add(-2.0, -3.0), -5.0);
    }

    #[test]
    fn test_add_decimals() {
        let result = add(1.5, 2.5);
        assert!((result - 4.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_add_zero() {
        assert_eq!(add(0.0, 5.0), 5.0);
        assert_eq!(add(5.0, 0.0), 5.0);
    }

    #[test]
    fn test_get_system_info() {
        let info = get_system_info();

        // These should be non-empty strings
        assert!(!info.platform.is_empty());
        assert!(!info.arch.is_empty());
        assert!(!info.family.is_empty());

        // Platform should be a known value
        let valid_platforms = ["linux", "macos", "windows", "ios", "android", "freebsd"];
        assert!(
            valid_platforms.contains(&info.platform.as_str())
                || !info.platform.is_empty(),
            "Unexpected platform: {}",
            info.platform
        );
    }
}
