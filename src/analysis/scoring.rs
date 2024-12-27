pub fn reversed_elu(input: f64) -> f64 {
    return -(-input).exp() + 1.0;
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