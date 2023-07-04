# Order-Preserving Encryption (OPE) Project

## Overview
This project is a Rust implementation of Order-Preserving Encryption (OPE). OPE is a cryptographic technique for encrypting ordered data in such a way that the order is preserved under encryption. This enables range queries and other order-based operations to be performed on the encrypted data without requiring decryption first.

## Features
- Implements OPE algorithms to encrypt and decrypt data while preserving order.
- Allows range queries on encrypted data.
- Provides a command-line interface for easy interaction with the library.
- Secure and efficient, with minimal overhead.
- Well-documented codebase for better understanding and contribution.

## Prerequisites
- Rust (latest stable version)
- Cargo (comes with Rust)

## Installation

`cargo add liserk-ope`

## Usage

### As a Library

Include this crate in your Rust project by adding the following in your `Cargo.toml`:

```toml
[dependencies]
ope_project = { path = "path_to_this_project" }
```

Then you can use the OPE library in your code like this:

```rust
use liserk_ope::*;

fn main() {
    let key = b"test_key";
    let ope = Ope::new(key, 0, 20).unwrap();

    let plaintext = 5;
    let ciphertext = ope.encrypt(plaintext).unwrap();
    assert!(ope.out_range.contains(ciphertext));

    let decrypted = ope.decrypt(ciphertext).unwrap();
    assert_eq!(plaintext, decrypted);
}
```

## Research Acknowledgements

This project owes its existence to the groundbreaking research in the field of Order-Preserving Encryption (OPE). Notably, we would like to mention the seminal paper "Order-Preserving Symmetric Encryption" by Boldyreva, Chenette, Lee, and O’Neill, which laid the foundation for practical OPE schemes. This paper, published in 2009, thoroughly examined the security properties and use cases for OPE and has since been a cornerstone for subsequent developments in this field. Another critical work is "Practical Order-Revealing Encryption with Limited Leakage" by Chenette, Lewi, Weis, and Wu, which offers a different approach to OPE that minimizes leakage. Their contributions have been invaluable to the cryptographic community and have directly inspired the algorithms and methodologies implemented in this project. We would also like to acknowledge various other papers and research articles that have collectively contributed to the advancement of OPE techniques. Through their rigorous research, they have made it possible to handle ordered data securely and efficiently.

## Contributing

Contributions to this project are welcome. If you find a bug or think of a new feature, please create an issue. If you would like to contribute code, please fork the repository and submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements
This project is developed and maintained by contributors who are passionate about security and cryptography. Special thanks to everyone who has contributed to making this project possible.

This project is based on mathematical 
https://eprint.iacr.org/2012/624.pdf
