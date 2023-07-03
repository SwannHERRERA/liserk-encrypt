use crate::test_stats::{sample_hgd, sample_uniform};
use aes::{
    cipher::{generic_array::GenericArray, KeyInit},
    Aes128, Aes256,
};
use hmac::{Hmac, Ma, Mac};
use sha2::Sha256;

type AesCtr = Ctr128<Aes256, Pkcs7>;

const DEFAULT_IN_RANGE_START: i32 = 0;
const DEFAULT_IN_RANGE_END: i32 = 2i32.pow(15) - 1;
const DEFAULT_OUT_RANGE_START: i32 = 0;
const DEFAULT_OUT_RANGE_END: i32 = 2i32.pow(31) - 1;

#[derive(Debug, Clone, PartialEq)]
pub struct ValueRange {
    pub start: i32,
    pub end: i32,
}

impl ValueRange {
    pub fn new(start: i32, end: i32) -> Result<Self, OpeError> {
        if start > end {
            return Err(OpeError::InvalidRangeLimitsError);
        }
        Ok(ValueRange { start, end })
    }

    pub fn size(&self) -> i32 {
        self.end - self.start + 1
    }

    pub fn contains(&self, number: i32) -> bool {
        self.start <= number && number <= self.end
    }
}

struct Ope {
    key: Vec<u8>,
    in_range: ValueRange,
    out_range: ValueRange,
}

impl Ope {
    fn new(
        key: &[u8],
        in_range: Option<ValueRange>,
        out_range: Option<ValueRange>,
    ) -> Result<Self, OpeError> {
        let in_range = in_range
            .unwrap_or(ValueRange::new(DEFAULT_IN_RANGE_START, DEFAULT_IN_RANGE_END)?);
        let out_range = out_range
            .unwrap_or(ValueRange::new(DEFAULT_OUT_RANGE_START, DEFAULT_OUT_RANGE_END)?);

        if in_range.size() > out_range.size() {
            return Err(OpeError::OutOfRangeError);
        }

        Ok(Ope { key: key.to_vec(), in_range, out_range })
    }

    fn encrypt(&self, plaintext: i32) -> Result<i32, OpeError> {
        if !self.in_range.contains(plaintext) {
            return Err(OpeError::OutOfRangeError);
        }
        self.encrypt_recursive(plaintext, self.in_range.clone(), self.out_range.clone())
    }

    fn encrypt_recursive(
        &self,
        plaintext: i32,
        in_range: ValueRange,
        out_range: ValueRange,
    ) -> Result<i32, OpeError> {
        let in_size = in_range.size();
        let out_size = out_range.size();
        let in_edge = in_range.start.wrapping_sub(1);
        let out_edge = out_range.start.wrapping_sub(1);
        let mid = out_edge + ((out_size + 1) / 2);

        if in_range.size() == 1 {
            let mut coins = self.tape_gen(plaintext)?;
            let ciphertext = sample_uniform(out_range, &mut coins.iter()); // sample_uniform needs to be implemented
            return Ok(ciphertext);
        }

        let coins = self.tape_gen(mid)?;
        let x = sample_hgd(in_range, out_range, mid, &mut coins.iter()); // sample_hgd needs to be implemented

        if plaintext <= x {
            in_range = ValueRange::new(in_edge + 1, x)?;
            out_range = ValueRange::new(out_edge + 1, mid)?;
        } else {
            in_range = ValueRange::new(x + 1, in_edge + in_size)?;
            out_range = ValueRange::new(mid + 1, out_edge + out_size)?;
        }
        self.encrypt_recursive(plaintext, in_range, out_range)
    }

    fn decrypt(&self, ciphertext: i32) -> Result<i32, OpeError> {
        if !self.out_range.contains(ciphertext) {
            return Err(OpeError::OutOfRangeError);
        }
        self.decrypt_recursive(ciphertext, self.in_range.clone(), self.out_range.clone())
    }

    fn decrypt_recursive(
        &self,
        ciphertext: i32,
        in_range: ValueRange,
        out_range: ValueRange,
    ) -> Result<i32, OpeError> {
        // ... Similar to encrypt_recursive but with decryption logic

        todo!()
        // You'll need to implement the recursive decryption logic here,
        // which is similar to encrypt_recursive but works for decryption.
    }
    fn tape_gen(&self, data: i32) -> Result<Vec<u8>, OpeError> {
        let data = data.to_le_bytes();

        let mut hmac = <&[u8] as Mac>::new_from_slice(&self.key).unwrap();
        hmac.update(&data);
        let result = hmac.finalize().into_bytes();

        // Use AES in the CTR mode to generate a pseudo-random bit string
        type AesCtr = Ctr128<Aes128>;
        let cipher = Aes128::new(GenericArray::from_slice(&result[..16])); // Use the first 128 bits of the HMAC result
        let counter = GenericArray::from_slice(&[0u8; 16]); // Start counter from all 0s
        let mut block_mode = AesCtr::new(cipher, counter);

        let mut buffer = [0u8; 16]; // A buffer to encrypt and generate random bytes
        block_mode.encrypt(&mut buffer, 16); // Encrypt in-place

        Ok(buffer.to_vec())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum OpeError {
    InvalidRangeLimitsError,
    InvalidCiphertextError,
    OutOfRangeError,
    InvalidCoinError,
    NotEnoughCoinsError,
}
