/// Generate the Collatz sequence for a given starting number
pub fn collatz_sequence(mut n: u64) -> Vec<u64> {
    let mut sequence = vec![n];
    
    while n != 1 {
        n = if n % 2 == 0 {
            n / 2
        } else {
            3 * n + 1
        };
        sequence.push(n);
    }
    
    sequence
}

/// Generate predecessors of a number in the Collatz sequence
/// For a number n, predecessors are:
/// - 2*n (always exists)
/// - (n-1)/3 (exists only if n-1 is divisible by 3 and n%3==1)
pub fn collatz_predecessors(n: u64) -> Vec<u64> {
    let mut predecessors = vec![2 * n];
    
    // Check if (n-1)/3 is a valid predecessor
    if n > 1 && n % 3 == 1 && (n - 1) % 3 == 0 {
        let pred = (n - 1) / 3;
        // Only add odd predecessors (since 3n+1 must come from odd numbers)
        if pred % 2 == 1 {
            predecessors.push(pred);
        }
    }
    
    predecessors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collatz_sequence() {
        assert_eq!(collatz_sequence(1), vec![1]);
        assert_eq!(collatz_sequence(2), vec![2, 1]);
        assert_eq!(collatz_sequence(3), vec![3, 10, 5, 16, 8, 4, 2, 1]);
    }

    #[test]
    fn test_collatz_predecessors() {
        let preds = collatz_predecessors(1);
        assert!(preds.contains(&2));
        
        let preds = collatz_predecessors(4);
        assert!(preds.contains(&8));
    }
}
