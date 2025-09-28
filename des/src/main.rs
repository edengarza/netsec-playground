use rand::Rng;
use des::{initial_permutation, inverted_permutation, generate_key_schedule, encipher_block};

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

fn test_permutation_and_back() {
    // create the rng
    let mut rng = rand::rng();
    let mut failed = false;
    let test_count = 100;

    // test many values
    for test_idx in 0..test_count {
        // gen() generates a value between 0 and u64::MAX
        let value: u64 = rng.random();
        let permuted = initial_permutation(&value);
        let inverted = inverted_permutation(&permuted);

        // on issue, break with good prints
        if value != inverted {
            println!("[{}] Failed on value {}", test_idx, value);
            print_bytes(&value);
            print_bytes(&inverted);
            failed = true;
        }
    }

    // end print in case all is good
    if !failed {
        println!("Passed {} permutation tests successfully!", test_count);
    }
}

fn test_key_schedule() {
    // create the key
    let mut rng = rand::rng();
    let key: u64 = rng.random();
    println!("Using the following key:");
    print_bytes(&key);

    let key_schedule = generate_key_schedule(&key);

    //for i in 0..16 {
    //    println!("Iteration {}", i);
    //    print_bytes(&key_schedule[i]);
    //}
}

fn test_encipher() {
    // create a random key
    let mut rng = rand::rng();
    let key: u64 = rng.random();
    println!("Using the following key:");
    print_bytes(&key);

    // message to encipher
    let msg: u32 = 1;
    println!("Using the following message:");
    print_bytes(&(msg as u64));

    let ciphertext = encipher_block(&msg, &key);
    println!("Resulting ciphertext:");
    print_bytes(&(ciphertext as u64));
}

fn main() {
    println!("Howdy! Welcome to testing DES :3\n");
    test_permutation_and_back();
    test_key_schedule();
    test_encipher();
}
