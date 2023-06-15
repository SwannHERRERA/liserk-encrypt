use crate::{error::Error, settings::SETTINGS};
use aes_gcm_siv::{
    aead::{generic_array::GenericArray, Aead},
    Aes256GcmSiv, KeyInit,
};
use chrono::{prelude::*, Duration};
use pqc_kyber::*;
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

impl Certificate {
    /// care year % 4 BUT
    fn new(public_key: Vec<u8>) -> Self {
        let issuer = String::from("Stuga Cloud Certificate Authority");
        let server = String::from("Server");
        let now = Utc::now();
        let end_of_validity = now + Duration::days(365);
        Certificate {
            public_key,
            identity_info: server,
            issuer_info: issuer,
            signature: Vec::new(), // TODO: TMP shoud use falcon
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

    let cipher = Aes256GcmSiv::new(GenericArray::from_slice(&SETTINGS.cipher.key));

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
