//! Core Collatz mathematics using arbitrary-precision integers.
//!
//! This module implements all fundamental Collatz operations, including both
//! the standard (full-integer) Collatz sequence and the odd-only Collatz tree
//! described in `odd_collatz_tree.py`.
//!
//! # Standard Collatz
//!
//! The Collatz conjecture operates on positive integers:
//! - If `n` is even:  `n → n / 2`
//! - If `n` is odd:   `n → 3n + 1`
//!
//! # Odd Collatz Tree
//!
//! In the odd Collatz tree an odd number `m` is a *child* of odd number `n`
//! when applying the Collatz step `(3m + 1)` and then repeatedly halving the
//! even result eventually yields `n`.  Equivalently:
//!
//! ```text
//! 3m + 1 = n * 2^k   for some positive integer k
//! m = (n * 2^k - 1) / 3
//! ```
//!
//! Only values of `k` for which the right-hand side is a positive odd integer
//! are valid children.  Because `n * 2^k` is always even, `n * 2^k - 1` is
//! always odd, so whenever the expression is divisible by 3 the result is
//! automatically odd.
//!
//! Odd multiples of 3 have *no* children: `n ≡ 0 (mod 3)` means
//! `n * 2^k - 1 ≡ -1 (mod 3)` for every `k`, so no valid `k` exists.

use num_bigint::BigUint;
use num_traits::{One, Zero};

// ---------------------------------------------------------------------------
// Standard Collatz
// ---------------------------------------------------------------------------

/// Perform a single Collatz step on `n`.
///
/// - Even: `n → n / 2`
/// - Odd:  `n → 3n + 1`
///
/// # Panics
/// Panics if `n` is zero.
pub fn collatz_step(n: &BigUint) -> BigUint {
    assert!(!n.is_zero(), "collatz_step is not defined for zero");
    if n % 2u32 == BigUint::zero() {
        n / 2u32
    } else {
        n * 3u32 + BigUint::one()
    }
}

/// Generate the Collatz sequence starting from `n` until it reaches 1.
///
/// The returned vector begins with `n` and ends with `1`.
///
/// # Panics
/// Panics if `n` is zero.
pub fn collatz_sequence(mut n: BigUint) -> Vec<BigUint> {
    assert!(!n.is_zero(), "collatz_sequence is not defined for zero");
    let one = BigUint::one();
    let mut sequence = vec![n.clone()];
    while n != one {
        n = collatz_step(&n);
        sequence.push(n.clone());
    }
    sequence
}

/// Generate all predecessors of `n` in the Collatz graph.
///
/// For a number `n`, the predecessors are:
/// - `2 * n` (always exists, via the even halving step)
/// - `(n - 1) / 3` (exists only when `n ≡ 1 (mod 3)` and `(n-1)/3` is odd,
///   since the `3m + 1` step only applies to odd `m`)
pub fn collatz_predecessors(n: &BigUint) -> Vec<BigUint> {
    let mut predecessors = vec![n * 2u32];

    // (n - 1) / 3 is a valid predecessor when n > 1, n ≡ 1 (mod 3), and the
    // result is odd (guaranteeing it came from the 3m+1 branch).
    if *n > BigUint::one() && n % 3u32 == BigUint::one() {
        let pred = (n - BigUint::one()) / 3u32;
        if &pred % 2u32 == BigUint::one() {
            predecessors.push(pred);
        }
    }

    predecessors
}

// ---------------------------------------------------------------------------
// Odd Collatz Tree
// ---------------------------------------------------------------------------

/// Return `true` if the odd number `n` has any odd Collatz children.
///
/// Odd multiples of 3 have no children; all other odd numbers have
/// infinitely many.
pub fn has_odd_collatz_children(n: &BigUint) -> bool {
    n % 3u32 != BigUint::zero()
}

/// An iterator that yields the odd Collatz tree children of odd number `n`.
///
/// A child `m` satisfies `3m + 1 = n * 2^k` for some positive integer `k`,
/// i.e. `m = (n * 2^k - 1) / 3`.  Only integer (automatically odd) results
/// are yielded.  The trivial fixed-point case (`n = 1, k = 2 → child = 1`)
/// is skipped so the tree stays acyclic.
pub struct OddCollatzChildren {
    n: BigUint,
    k: u32,
}

impl OddCollatzChildren {
    fn new(n: BigUint) -> Self {
        OddCollatzChildren { n, k: 1 }
    }
}

impl Iterator for OddCollatzChildren {
    type Item = BigUint;

    fn next(&mut self) -> Option<BigUint> {
        let three = BigUint::from(3u32);
        loop {
            // numerator = n * 2^k - 1
            let numerator = (&self.n << self.k) - BigUint::one();
            self.k += 1;
            if &numerator % &three == BigUint::zero() {
                let child = numerator / &three;
                // Skip the n = 1 fixed point (child equals the parent)
                if child != self.n {
                    return Some(child);
                }
            }
        }
    }
}

/// Return an iterator that yields the odd Collatz tree children of `n`.
///
/// The children are produced in order of increasing `k` (the exponent in
/// `3m + 1 = n * 2^k`).  The iterator is infinite for any `n` that
/// satisfies [`has_odd_collatz_children`].
///
/// # Panics
/// The iterator is empty (but non-terminating) for odd multiples of 3.
/// Prefer checking [`has_odd_collatz_children`] first when needed.
pub fn odd_collatz_children(n: BigUint) -> OddCollatzChildren {
    OddCollatzChildren::new(n)
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

    // --- collatz_step ---

    #[test]
    fn test_collatz_step_even() {
        assert_eq!(collatz_step(&big(8)), big(4));
        assert_eq!(collatz_step(&big(10)), big(5));
    }

    #[test]
    fn test_collatz_step_odd() {
        assert_eq!(collatz_step(&big(3)), big(10));
        assert_eq!(collatz_step(&big(5)), big(16));
    }

    // --- collatz_sequence ---

    #[test]
    fn test_collatz_sequence_trivial() {
        assert_eq!(collatz_sequence(big(1)), vec![big(1)]);
    }

    #[test]
    fn test_collatz_sequence_two() {
        assert_eq!(collatz_sequence(big(2)), vec![big(2), big(1)]);
    }

    #[test]
    fn test_collatz_sequence_three() {
        assert_eq!(
            collatz_sequence(big(3)),
            vec![3, 10, 5, 16, 8, 4, 2, 1].into_iter().map(big).collect::<Vec<_>>()
        );
    }

    // --- collatz_predecessors ---

    #[test]
    fn test_collatz_predecessors_one() {
        let preds = collatz_predecessors(&big(1));
        assert!(preds.contains(&big(2)));
    }

    #[test]
    fn test_collatz_predecessors_four() {
        let preds = collatz_predecessors(&big(4));
        assert!(preds.contains(&big(8)));
    }

    #[test]
    fn test_collatz_predecessors_both_branches() {
        // collatz_predecessors(16) should include 32 (even) and 5 (odd: (16-1)/3 = 5)
        let preds = collatz_predecessors(&big(16));
        assert!(preds.contains(&big(32)));
        assert!(preds.contains(&big(5)));
    }

    // --- has_odd_collatz_children ---

    #[test]
    fn test_has_odd_collatz_children_true() {
        // Non-multiples of 3 have children
        for &n in &[1u64, 5, 7, 11, 13, 17, 19, 23, 25] {
            assert!(
                has_odd_collatz_children(&big(n)),
                "{n} should have odd Collatz children"
            );
        }
    }

    #[test]
    fn test_has_odd_collatz_children_false() {
        // Odd multiples of 3 have no children
        for &n in &[3u64, 9, 15, 21, 27] {
            assert!(
                !has_odd_collatz_children(&big(n)),
                "{n} should have no odd Collatz children"
            );
        }
    }

    // --- odd_collatz_children ---

    #[test]
    fn test_odd_collatz_children_of_one() {
        // First 5 children of 1 are [5, 21, 85, 341, 1365]
        let children: Vec<BigUint> = odd_collatz_children(big(1)).take(5).collect();
        assert_eq!(
            children,
            vec![5u64, 21, 85, 341, 1365].into_iter().map(big).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_odd_collatz_children_of_five() {
        // Verify that applying one Collatz step to each child and halving
        // repeatedly eventually reaches the parent (5).
        let parent = big(5);
        for child in odd_collatz_children(big(5)).take(5) {
            let mut n = collatz_step(&child); // 3*child + 1 (child is odd)
            while &n % 2u32 == BigUint::zero() {
                n = n / 2u32;
            }
            assert_eq!(n, parent);
        }
    }

    #[test]
    fn test_odd_collatz_children_all_odd() {
        // All children must be odd
        for child in odd_collatz_children(big(1)).take(10) {
        assert_eq!(&child % 2u32, BigUint::one());
        }
    }
}
