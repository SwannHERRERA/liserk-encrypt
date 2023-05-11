use tfhe::shortint::prelude::*;
use tikv_client::{Error, RawClient};

fn encrypt_test() {
    // Generate a set of client/server keys
    // with 2 bits of message and 2 bits of carry
    let (client_key, server_key) = gen_keys(PARAM_MESSAGE_2_CARRY_2);
    // COmment ça se fait que j'ia la clef client et server

    let msg1 = 3;
    let msg2 = 2;

    // Encrypt two messages using the (private) client key:
    let ct_1 = client_key.encrypt(msg1);
    let ct_2 = client_key.encrypt(msg2);

    // Homomorphically compute an addition
    let ct_add = server_key.unchecked_add(&ct_1, &ct_2);

    // Define the Hamming weight function
    // f: x -> sum of the bits of x
    let f = |x: u64| x.count_ones() as u64;

    // Generate the accumulator for the function
    let acc = server_key.generate_accumulator(f);

    // Compute the function over the ciphertext using the PBS
    let ct_res = server_key.apply_lookup_table(&ct_add, &acc);

    // Decrypt the ciphertext using the (private) client key
    let output = client_key.decrypt(&ct_res);
    assert_eq!(output, f(msg1 + msg2));
    println!("{}, {}", msg1, msg2);
    println!("{}", output);
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = RawClient::new(vec!["0.0.0.0:2379"]).await?;

    let key = "Hello".to_owned();
    let value = "RawKV".to_owned();

    const LIMIT: u32 = 1000;
    client.put("k1".to_owned(), "v1".to_owned()).await?;
    client.put("k2".to_owned(), "v2".to_owned()).await?;
    client.put("k3".to_owned(), "v3".to_owned()).await?;
    client.put("k4".to_owned(), "v4".to_owned()).await?;
    let result = client.scan("k1".to_owned().."k5".to_owned(), LIMIT).await?;
    println!("{:?}", result);

    encrypt_test();

    Ok(())
}
