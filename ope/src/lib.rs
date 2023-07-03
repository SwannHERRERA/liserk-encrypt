pub mod hgd;
pub mod ope;
pub mod simplified_version;
pub mod stats;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::ope::{Ope, ValueRange};

    #[test]
    fn test_value_range_new() {
        assert!(ValueRange::new(5, 3).is_err());
        assert_eq!(ValueRange::new(3, 5), Ok(ValueRange { start: 3, end: 5 }));
    }

    #[test]
    fn test_value_range_size() {
        let range = ValueRange::new(3, 5).unwrap();
        assert_eq!(range.size(), 3);

        let range = ValueRange::new(-3, -1).unwrap();
        assert_eq!(range.size(), 3);
    }

    #[test]
    fn test_value_range_contains() {
        let range = ValueRange::new(3, 5).unwrap();
        assert!(range.contains(4));
        assert!(!range.contains(6));
    }

    #[test]
    #[should_panic]
    fn test_ope_new() {
        let key = b"test_key";
        assert!(Ope::new(key, Some(ValueRange::new(5, 3).unwrap()), None).is_err());
        assert!(Ope::new(key, None, None).is_err());
    }

    #[test]
    fn test_ope_encrypt_decrypt() {
        let key = b"test_key";
        let ope = Ope::new(key, None, None).unwrap();

        let plaintext = 5;
        let ciphertext = ope.encrypt(plaintext).unwrap();
        assert!(ope.out_range.contains(ciphertext));

        let decrypted = ope.decrypt(ciphertext).unwrap();
        assert_eq!(plaintext, decrypted);
    }
}
