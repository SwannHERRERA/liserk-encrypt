pub fn byte_to_bitstring(byte: u8) -> Vec<u8> {
    let mut bits = Vec::new();
    for i in 0..8 {
        bits.push((byte >> (7 - i)) & 1);
    }
    bits
}

pub fn data_to_byte_list(data: &Vec<u8>) -> Vec<u8> {
    data.clone()
}

pub fn str_to_bitstring(data: Vec<u8>) -> Vec<u8> {
    let mut bit_list = Vec::new();
    for byte in data_to_byte_list(&data) {
        bit_list.extend(byte_to_bitstring(byte));
    }
    bit_list
}
