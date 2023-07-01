use crate::simplified_version::decrypt_ope;

use crate::simplified_version::encrypt_ope;

mod hgd;
mod simplified_version;
mod stat;

fn main() {
    let number_to_encrypt = 12.0;
    let encrypted_number_1 = encrypt_ope(number_to_encrypt);
    let number_to_encrypt = 19.0;
    let encrypted_number_2 = encrypt_ope(number_to_encrypt);
    println!("Encrypted number 1: {}", encrypted_number_1);
    println!("Encrypted number 2: {}", encrypted_number_2);
    println!("19 > 12: {}", encrypted_number_1 < encrypted_number_2);

    let number_to_encrypt = -12.0;
    let encrypted_number_1 = encrypt_ope(number_to_encrypt);
    let number_to_encrypt = -19.0;
    let encrypted_number_2 = encrypt_ope(number_to_encrypt);
    println!("Encrypted number 1: {}", encrypted_number_1);
    println!("Encrypted number 2: {}", encrypted_number_2);
    println!("-12 > -19: {}", encrypted_number_1 > encrypted_number_2);

    let decrypt_number_2 = decrypt_ope(encrypted_number_2);
    println!("Decrypted number 2: {}", decrypt_number_2.unwrap_or(2.0));
}
