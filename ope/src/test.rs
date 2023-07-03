use crate::test_stats::{sample_hgd, sample_uniform};
use aes::Aes256;
use hmac::{Hmac, Mac, NewMac};
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
    fn tape_gen(&self, data: &str) -> impl Iterator<Item = bool> {
        let mut hmac =
            Hmac::new(MessageDigest::sha256(), self.key).expect("Failed to create HMAC");

        hmac.update(data.as_bytes());

        assert_eq!(hmac.size(), 32, "HMAC digest size must be 32 bytes");

        let digest = hmac.finalize().into_bytes();

        let aes = Aes256::new(&digest);
        let mut ctr = Ctr128::<Aes256, NoPadding>::new(aes, &vec![0; 16]);

        let mut buffer = vec![0; 16];

        std::iter::from_fn(move || {
            ctr.encrypt(&mut buffer, 16);

            let mut bits = Vec::new();
            for byte in buffer.iter() {
                for i in 0..8 {
                    bits.push((byte >> i) & 1 == 1);
                }
            }

            if let Some(bit) = bits.pop() {
                Some(bit)
            } else {
                None
            }
        })
    }
    fn tape_gen(&self, data: i32) -> Result<Vec<u8>, OpeError> {
        let data = data.to_le_bytes();
        let mut hmac = Hmac::<Sha256>::new_from_slice(&self.key).unwrap();
        hmac.update(&data);
        let result = hmac.finalize().into_bytes();
        todo!()

        // The next steps involve using AES in CTR mode to generate a pseudo-random bit string
        // which is similar to what the Python code is doing. You can use the `aes` and `block-modes`
        // crate for this purpose.

        // ... rest of tape_gen implementation
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
