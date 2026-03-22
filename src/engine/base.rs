//! Base conversion utilities for arbitrary-precision integers.
//!
//! This module provides functions to display [`BigUint`] values in various
//! numeral bases, including base 2, 3, 6, 12, and 24.

use num_bigint::BigUint;
use num_traits::{ToPrimitive, Zero};

/// Digit characters used for base conversion, covering bases up to 36.
///
/// Digits 0–9 use ASCII numerals; digits 10–35 use uppercase letters A–Z.
const DIGITS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// Convert a [`BigUint`] to its string representation in the given `base`.
///
/// Digits above 9 are represented with uppercase letters: `A` = 10, `B` = 11,
/// and so on up to `Z` = 35.  This supports all the commonly useful bases 2,
/// 3, 6, 12, and 24.
///
/// | `n` | base 2   | base 3 | base 6 | base 12 | base 24 |
/// |-----|----------|--------|--------|---------|---------|
/// | 42  | `101010` | `1120` | `110`  | `36`    | `1I`    |
///
/// # Panics
/// Panics if `base` is less than 2 or greater than 36.
pub fn to_base_string(n: &BigUint, base: u32) -> String {
    assert!(base >= 2 && base <= 36, "base must be between 2 and 36");

    if n.is_zero() {
        return "0".to_string();
    }

    let base_big = BigUint::from(base);
    let mut digits = Vec::new();
    let mut current = n.clone();

    while !current.is_zero() {
        let remainder = (&current % &base_big)
            .to_u32()
            .expect("remainder is always less than base");
        digits.push(DIGITS[remainder as usize]);
        current /= &base_big;
    }

    digits.reverse();
    String::from_utf8(digits).expect("digits are valid UTF-8")
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn big(n: u64) -> BigUint {
        BigUint::from(n)
    }

    // --- to_base_string ---

    #[test]
    fn test_to_base_string_zero() {
        for base in [2, 3, 6, 12, 24] {
            assert_eq!(to_base_string(&big(0), base), "0");
        }
    }

    #[test]
    fn test_to_base_string_base2() {
        assert_eq!(to_base_string(&big(1), 2), "1");
        assert_eq!(to_base_string(&big(42), 2), "101010");
        assert_eq!(to_base_string(&big(255), 2), "11111111");
    }

    #[test]
    fn test_to_base_string_base3() {
        assert_eq!(to_base_string(&big(9), 3), "100");
        assert_eq!(to_base_string(&big(42), 3), "1120");
    }

    #[test]
    fn test_to_base_string_base6() {
        assert_eq!(to_base_string(&big(6), 6), "10");
        assert_eq!(to_base_string(&big(42), 6), "110");
    }

    #[test]
    fn test_to_base_string_base12() {
        assert_eq!(to_base_string(&big(10), 12), "A");
        assert_eq!(to_base_string(&big(11), 12), "B");
        assert_eq!(to_base_string(&big(12), 12), "10");
        assert_eq!(to_base_string(&big(42), 12), "36");
    }

    #[test]
    fn test_to_base_string_base24() {
        assert_eq!(to_base_string(&big(10), 24), "A");
        assert_eq!(to_base_string(&big(23), 24), "N");
        assert_eq!(to_base_string(&big(24), 24), "10");
        assert_eq!(to_base_string(&big(42), 24), "1I");
    }

    #[test]
    fn test_to_base_string_large_number() {
        // 2^32 in base 2 is "1" followed by 32 zeros
        let n = BigUint::from(1u64 << 32);
        let base2 = to_base_string(&n, 2);
        assert_eq!(base2, "1".to_string() + &"0".repeat(32));
    }
}
