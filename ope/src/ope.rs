use std::cell::RefCell;

use crate::stats::{sample_hgd, sample_uniform};
use hmac::Hmac;
use sha2::Sha256;

pub type HmacSha256 = Hmac<Sha256>;

pub const DEFAULT_IN_RANGE_START: i32 = 0;
pub const DEFAULT_IN_RANGE_END: i32 = 2i32.pow(15) - 1;
pub const DEFAULT_OUT_RANGE_START: i32 = 0;
pub const DEFAULT_OUT_RANGE_END: i32 = 2i32.pow(15) - 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpeError {
    InvalidRangeLimitsError,
    InvalidCiphertextError,
    OutOfRangeError,
    InvalidCoinError,
    NotEnoughCoinsError,
}

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

pub struct Ope {
    pub key: Vec<u8>,
    pub in_range: ValueRange,
    pub out_range: ValueRange,
}

impl Ope {
    pub fn new(
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

    pub fn encrypt(&self, plaintext: i32) -> Result<i32, OpeError> {
        if !self.in_range.contains(plaintext) {
            return Err(OpeError::OutOfRangeError);
        }
        self.encrypt_recursive(plaintext, self.in_range.clone(), self.out_range.clone())
    }

    fn encrypt_recursive(
        &self,
        plaintext: i32,
        mut in_range: ValueRange,
        mut out_range: ValueRange,
    ) -> Result<i32, OpeError> {
        let in_size = in_range.size();
        let out_size = out_range.size();
        let in_edge = in_range.start.wrapping_sub(1);
        let out_edge = out_range.start.wrapping_sub(1);
        let mid = out_edge + ((out_size + 1) / 2);

        if in_range.size() == 1 {
            let coins = RefCell::new(self.tape_gen(plaintext));
            let ciphertext = sample_uniform(out_range, Box::new(coins));
            return Ok(ciphertext);
        }

        let coins = self.tape_gen(mid);
        let x = sample_hgd(in_range, out_range, mid, Box::new(coins));

        if plaintext <= x {
            in_range = ValueRange::new(in_edge + 1, x)?;
            out_range = ValueRange::new(out_edge + 1, mid)?;
        } else {
            in_range = ValueRange::new(x + 1, in_edge + in_size)?;
            out_range = ValueRange::new(mid + 1, out_edge + out_size)?;
        }
        self.encrypt_recursive(plaintext, in_range, out_range)
    }

    pub fn decrypt(&self, ciphertext: i32) -> Result<i32, OpeError> {
        if !self.out_range.contains(ciphertext) {
            return Err(OpeError::OutOfRangeError);
        }
        self.decrypt_recursive(ciphertext, self.in_range.clone(), self.out_range.clone())
    }

    fn decrypt_recursive(
        &self,
        ciphertext: i32,
        mut in_range: ValueRange,
        mut out_range: ValueRange,
    ) -> Result<i32, OpeError> {
        let in_size = in_range.size();
        let out_size = out_range.size();
        let in_edge = in_range.start.wrapping_sub(1);
        let out_edge = out_range.start.wrapping_sub(1);
        let mid = out_edge + ((out_size + 1) / 2);

        if out_range.size() == 1 {
            let coins = RefCell::new(self.tape_gen(ciphertext));
            let plaintext = sample_uniform(in_range, Box::new(coins)); // sample_uniform needs to be implemented
            return Ok(plaintext);
        }

        let coins = self.tape_gen(mid);
        let x = sample_hgd(in_range, out_range, mid, Box::new(coins)); // sample_hgd needs to be implemented

        if ciphertext <= mid {
            out_range = ValueRange::new(out_edge + 1, mid)?;
            in_range = ValueRange::new(in_edge + 1, x)?;
        } else {
            out_range = ValueRange::new(mid + 1, out_edge + out_size)?;
            in_range = ValueRange::new(x + 1, in_edge + in_size)?;
        }
        self.decrypt_recursive(ciphertext, in_range, out_range)
    }

    fn tape_gen(&self, data: i32) -> impl Iterator<Item = bool> {
        let data_bytes = data.to_le_bytes();
        let mut bits = Vec::new();

        for &byte in data_bytes.iter() {
            for i in 0..8 {
                bits.push((byte >> i) & 1 == 1);
            }
        }

        bits.into_iter()
    }
}
