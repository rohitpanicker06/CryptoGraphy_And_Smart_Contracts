use hex;
use sha2::{Sha256, Digest};
use rand::RngCore;


fn main() {
    let id = "ED00AF5F774E4135E7746419FEB65DE8AE17D6950C95CEC3891070FBB5B03C78";
    let id_decoded = hex::decode(id).expect("Unable to decode hex into bytes");


    loop {
        //initializing array x with 32 elements where each element is a type u8 (unsigned 8 bit) -> also initializing them with 0
        let mut byte_array_for_holding_x:[u8;32] = [0; 32];
        rand::thread_rng().fill_bytes(&mut byte_array_for_holding_x);

        let mut vector_for_concatenation = byte_array_for_holding_x.to_vec();
        vector_for_concatenation.extend_from_slice(&id_decoded);

        let hashed_value = Sha256::digest(&vector_for_concatenation);


        if hashed_value.iter().any(|&byte| byte == 0x2F) {
            println!("hash containing 0x2F (47): {:?}", hashed_value);
            let x_hex = hex::encode(byte_array_for_holding_x);
            println!("Answer : -> x in hex (hash-encoded): {}", x_hex);
            break;
        }
    }
}
