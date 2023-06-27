use rug::Float;

pub const TOLERANCE: f64 = 1e-5;

fn calculate_hypergeometric_sum(
    length: f64,
    input_number: f64,
    probability: f64,
) -> Float {
    if probability <= 0.0 || probability >= 1.0 {
        panic!("Probability must be between 0 and 1.");
    }

    let precision = 200; // précision en bits
    let q = Float::with_val(precision, 1.0) - Float::with_val(precision, probability);
    let mean =
        Float::with_val(precision, length) * Float::with_val(precision, probability);
    let residual = Float::with_val(precision, length) * q;

    let mut numerator = Float::with_val(precision, 1.0);
    let mut factorial = Float::with_val(precision, 1.0);
    for j in 1..=input_number as i32 {
        numerator *= (mean.clone() + 1.0 - Float::with_val(precision, j))
            * (residual.clone() + 1.0 - Float::with_val(precision, j));
        factorial *= Float::with_val(precision, j);
    }

    if factorial.is_zero() {
        panic!("Factorial is zero, input may be too large.");
    }

    let mut hypergeometric_term = numerator / factorial;
    let mut sum = Float::with_val(precision, 0.0);
    let mut number = Float::with_val(precision, input_number);

    for _ in 0..10000 {
        sum += hypergeometric_term.clone();

        let next_number: Float = number.clone() + 1.0;
        let next_term: Float = (mean.clone() - number.clone())
            * (residual.clone() - number.clone())
            / (next_number.clone())
            / (mean.clone() + residual.clone() - next_number.clone() - 1.0);
        if next_term.is_infinite() || next_term.is_nan() {
            break;
        }

        hypergeometric_term = next_term;
        number = next_number;
    }

    let scaling_factor = Float::with_val(precision, 1e12);
    sum *= scaling_factor;
    sum.floor()
}

pub fn encrypt_ope(input_number: f64) -> Float {
    const KEY_SPACE_LENGTH: f64 = 16_777_216.0; // Can be any positive value representing the length of the key space
    const PROBABILITY: f64 = 0.5; // A probability, can be any value between 0 and 1

    calculate_hypergeometric_sum(KEY_SPACE_LENGTH, input_number, PROBABILITY)
}

pub fn decrypt_ope(
    encrypted_number: Float,
    key_space_length: f64,
    probability: f64,
) -> Option<f64> {
    for i in 0..key_space_length as i32 {
        let input_number = i as f64;
        let result =
            calculate_hypergeometric_sum(key_space_length, input_number, probability);
        let diff = (result - encrypted_number.clone()).abs();

        if diff < Float::with_val(200, TOLERANCE) {
            return Some(input_number);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_preserving_encryption() {
        let a = 10.0;
        let b = 20.0;
        let c = 30.0;

        let encrypted_a = encrypt_ope(a);
        let encrypted_b = encrypt_ope(b);
        let encrypted_c = encrypt_ope(c);

        assert!(encrypted_a < encrypted_b);
        assert!(encrypted_b < encrypted_c);
    }

    #[test]
    fn test_with_negative_number() {
        let a = -10.0;
        let b = -20.0;
        let c = -30.0;

        let encrypted_a = encrypt_ope(a);
        let encrypted_b = encrypt_ope(b);
        let encrypted_c = encrypt_ope(c);

        assert!(encrypted_a > encrypted_b);
        assert!(encrypted_b > encrypted_c);
    }

    #[test]
    fn test_edge_cases() {
        let a = 6_777_216.0;
        let b = 6_777_215.0;

        let encrypted_a = encrypt_ope(a);
        let encrypted_b = encrypt_ope(b);

        assert!(encrypted_a > encrypted_b);
    }

    #[test]
    fn test_same_input() {
        let a = 42.0;
        let encrypted_a1 = encrypt_ope(a);
        let encrypted_a2 = encrypt_ope(a);

        assert_eq!(encrypted_a1, encrypted_a2);
    }
}
