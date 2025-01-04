pub fn reversed_elu(input: f64) -> f64 {
    let base = f64::exp(1.0);
    return -base.powf(-input) + 1.0;
}

/// Check if score1 dominates score2,
/// i.e. score1 is better than score2 in all dimensions
pub fn dominates<const N: usize>(
    score1: &[f64; N],
    score2: &[f64; N],
) -> bool {
    for (s1, s2) in score1.iter().zip(score2.iter()) {
        if s1 < s2 {
            return false;
        }
    }
    true
}

/// Compare two scores, returning the net number of dimensions in which score1 is better than score2
pub fn is_better<const N: usize>(
    score1: &[f64; N],
    score2: &[f64; N],
) -> isize {
    let mut count = 0;
    for (s1, s2) in score1.iter().zip(score2.iter()) {
        if s1 > s2 {
            count += 1;
        } else if s1 < s2 {
            count -= 1;
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_compare_correctness() {
        let score1 = [1.0, 2.0, 3.0];
        let score2 = [1.0, 2.0, 3.0];
        assert_eq!(is_better(&score1, &score2), 0);
        assert_eq!(is_better(&score2, &score1), 0);

        let score1 = [1.0, 2.0, 3.0];
        let score2 = [1.0, 2.0, 2.0];
        assert_eq!(is_better(&score1, &score2), 1);
        assert_eq!(is_better(&score2, &score1), -1);

        let score1 = [f64::NAN, 2.0, 3.0];
        let score2 = [1.0, 2.0, 2.0];
        assert_eq!(is_better(&score1, &score2), 1);
        assert_eq!(is_better(&score2, &score1), -1);
    }
}