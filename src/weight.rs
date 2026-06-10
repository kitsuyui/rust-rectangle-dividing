use num_traits::{Num, NumAssignOps, NumOps};
// weights are just Vec<T>

pub(crate) fn normalize_weights<T>(weights: &[T]) -> Vec<T>
where
    T: Copy + Num + NumAssignOps + NumOps + PartialOrd + for<'a> std::iter::Sum<&'a T>,
{
    if weights.is_empty() {
        return vec![];
    }

    // Zero-sum: treat all weights as equal (handles [-1, 1] style cancel-out).
    let sum: T = weights.iter().sum();
    if sum == T::zero() {
        let count = weights.iter().fold(T::zero(), |acc, _| acc + T::one());
        return weights.iter().map(|_| T::one() / count).collect();
    }

    // Clamp negative weights to zero so the caller never receives negative proportions
    // (e.g. [-0.5, 1.5] would otherwise produce a negative-width rectangle).
    let clamped: Vec<T> = weights
        .iter()
        .map(|&w| if w < T::zero() { T::zero() } else { w })
        .collect();
    let clamped_sum: T = clamped.iter().sum();
    if clamped_sum == T::zero() {
        // All non-negative weights are zero: fall back to equal distribution.
        let count = clamped.iter().fold(T::zero(), |acc, _| acc + T::one());
        return clamped.iter().map(|_| T::one() / count).collect();
    }

    clamped.iter().map(|&w| w / clamped_sum).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_weights() {
        let weights = vec![1.0, 1.0, 1.0, 1.0];
        let normalized = normalize_weights(&weights);
        assert_eq!(normalized, vec![0.25, 0.25, 0.25, 0.25]);

        let weights = vec![1.0, 2.0, 3.0, 4.0];
        let normalized = normalize_weights(&weights);
        assert_eq!(normalized, vec![0.1, 0.2, 0.3, 0.4]);
    }

    #[test]
    fn test_normalize_zero_sum_weights() {
        let weights = vec![0.0, 0.0, 0.0, 0.0];
        let normalized = normalize_weights(&weights);
        assert_eq!(normalized, vec![0.25, 0.25, 0.25, 0.25]);

        let weights = vec![-1.0, 1.0];
        let normalized = normalize_weights(&weights);
        assert_eq!(normalized, vec![0.5, 0.5]);
    }

    #[test]
    fn test_normalize_mixed_sign_weights() {
        // Non-zero sum with a negative element: negative is clamped to zero.
        let weights = vec![-0.5f64, 1.5];
        let normalized = normalize_weights(&weights);
        assert!(
            normalized.iter().all(|&w| w >= 0.0),
            "all normalized must be >= 0"
        );
        assert!((normalized.iter().sum::<f64>() - 1.0).abs() < 1e-10);
        assert_eq!(normalized[0], 0.0);
        assert!((normalized[1] - 1.0).abs() < 1e-10);

        // All-negative weights: clamp to zero → equal distribution.
        let weights = vec![-1.0f64, -2.0, -3.0];
        let normalized = normalize_weights(&weights);
        assert!(normalized.iter().all(|&w| w >= 0.0));
        for w in &normalized {
            assert!((*w - 1.0 / 3.0).abs() < 1e-10);
        }
    }
}
