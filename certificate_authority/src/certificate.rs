use crate::{error::Error, settings::SETTINGS};
use aes_gcm_siv::{
    aead::{generic_array::GenericArray, Aead},
    Aes256GcmSiv, KeyInit,
};
use chrono::{prelude::*, Duration};
use pqc_kyber::*;
use pqcrypto_falcon::falcon512;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Certificate {
    pub public_key: Vec<u8>,
    pub identity_info: String,
    pub issuer_info: String,
    pub signature: Vec<u8>,
    pub valid_from: DateTime<Utc>,
    pub valid_to: DateTime<Utc>,
    pub serial_number: Uuid,
    pub cipher_suits: Vec<String>,
}

fn as_bytes<T>(input: &T) -> &[u8] {
    let ptr = input as *const T as *const u8;
    unsafe { std::slice::from_raw_parts(ptr, std::mem::size_of::<T>()) }
}

impl Certificate {
    /// care year % 4 BUT
    fn new(public_key: Vec<u8>) -> Self {
        let issuer = String::from("Stuga Cloud Certificate Authority");
        let server = String::from("Server");
        let now = Utc::now();
        let end_of_validity = now + Duration::days(365);
        let (_falcon_public_key, secret_key) = falcon512::keypair();
        let sign = falcon512::sign(&public_key, &secret_key);
        let sign_as_bytes = as_bytes(&sign);

        Certificate {
            public_key,
            identity_info: server,
            issuer_info: issuer,
            signature: sign_as_bytes.to_vec(),
            valid_from: now,
            valid_to: end_of_validity,
            serial_number: Uuid::new_v4(),
            cipher_suits: vec![String::from("kyber768"), String::from("falcon")],
        }
    }
}

pub fn create_certificate() -> Result<(), Error> {
    let mut rng = rand::thread_rng();
    let alice_keys = keypair(&mut rng);

    let cipher = Aes256GcmSiv::new(GenericArray::from_slice(&SETTINGS.cipher.aes_key));

    let mut nonce = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce);

    let ciphertext = cipher
        .encrypt(GenericArray::from_slice(&nonce), alice_keys.secret.as_ref())
        .expect("encryption failure!");

    let certificate = Certificate::new(alice_keys.public.to_vec());
    store_certificate(&SETTINGS.cipher.certificates_path, certificate)?;
    store_kyber_private_key(&SETTINGS.cipher.certificates_path, ciphertext)?;

    Ok(())
}

fn store_certificate(path: &String, certificate: Certificate) -> Result<(), Error> {
    let file_path = format!("{}certificate.crt", path);
    println!("path file: {}", file_path);
    let mut file = File::create(file_path)?;
    let certificate_as_string = toml::to_string(&certificate)?;
    file.write_all(certificate_as_string.as_bytes())?;
    Ok(())
}

//fn store_flacon_private_key() {}

fn store_kyber_private_key(path: &String, ciphertext: Vec<u8>) -> Result<(), Error> {
    let file_path = format!("{}encrypted.kyber", path);
    println!("path file: {}", file_path);
    let mut file = File::create(file_path)?;
    file.write_all(&ciphertext)?;
    Ok(())
}
