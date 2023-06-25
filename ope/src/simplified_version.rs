fn calculate_hypergeometric_sum(length: f64, input_number: f64, probability: f64) -> f64 {
    if probability <= 0.0 || probability >= 1.0 {
        panic!("Probability must be between 0 and 1.");
    }

    let q = 1.0 - probability;
    let mean = length * probability;
    let residual = length * q;

    let mut numerator = 1.0;
    let mut factorial = 1.0;
    for j in 1..=input_number as i32 {
        numerator *= (mean + 1.0 - j as f64) * (residual + 1.0 - j as f64);
        factorial *= j as f64;
    }

    if factorial == 0.0 {
        panic!("Factorial is zero, input may be too large.");
    }

    let mut hypergeometric_term = numerator / factorial;
    let mut sum = 0.0;
    let mut number = input_number;

    for _ in 0..10000 {
        sum += hypergeometric_term;

        let next_number = number + 1.0;
        let next_term = (mean - number) * (residual - number)
            / (next_number)
            / (mean + residual - next_number - 1.0);
        if next_term.is_infinite() || next_term.is_nan() {
            break;
        }

        hypergeometric_term = next_term;
        number = next_number;
    }

    let scaling_factor = 1e12;
    sum *= scaling_factor;
    sum.floor()
}

pub fn encrypt_ope(input_number: f64) -> f64 {
    const KEY_SPACE_LENGTH: f64 = 16_777_216.0; // Can be any positive value representing the length of the key space
    const PROBABILITY: f64 = 0.5; // A probability, can be any value between 0 and 1

    calculate_hypergeometric_sum(KEY_SPACE_LENGTH, input_number, PROBABILITY)
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
        let a = 21.0;
        let b = 32.0;

        let encrypted_a = encrypt_ope(a);
        let encrypted_b = encrypt_ope(b);

        assert!(encrypted_a < encrypted_b);
    }

    #[test]
    fn test_same_input() {
        let a = 42.0;
        let encrypted_a1 = encrypt_ope(a);
        let encrypted_a2 = encrypt_ope(a);

        assert_eq!(encrypted_a1, encrypted_a2);
    }
}
