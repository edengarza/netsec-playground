use rand::Rng;
use des::{encipher_block, decipher_block, generate_key_schedule};

fn print_bytes(value: &u64) {
    // print out one byte per line
    for i in (0..8).rev() {
        let byte_to_print = ((value >> (i * 8)) & 0xFF) as u8;

        // print each bit in the byte
        for j in (0..8).rev() {
            let bit = (byte_to_print >> j) & 1;
            if bit == 1 {
                print!("1");
            } else {
                print!("0");
            }
        }

        // print a newline after each byte
        println!();
    }

    // print a newline after the block
    println!();
}
fn encrypt_and_decrypt()
{
    // create a random key
    let mut rng = rand::rng();
    let key: u64 = rng.random();
    println!("Using the following key:");
    print_bytes(&key);

    // message to encipher
    let msg: u64 = 1;
    println!("Using the following message: {}", msg);
    print_bytes(&msg);

    // generate the key schedule
    let ks = generate_key_schedule(&key);

    // encrypt the message
    let encrypted_msg = encipher_block(&msg, &ks);
    println!("Encrypted message: {}", encrypted_msg);
    print_bytes(&encrypted_msg);

    // decrypt the message
    let decrypted_msg = decipher_block(&encrypted_msg, &ks);
    println!("Decrypted message: {}", decrypted_msg);
    print_bytes(&decrypted_msg);
}

fn main() {
    println!("Howdy! Welcome to testing DES :3\n");
    encrypt_and_decrypt();
}
