use des::{initial_permutation, inverted_permutation};

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
}

fn main() {
    println!("Howdy! Welcome to testing DES :3\n");

    let test: u64 = 1;
    println!("The value being permuted and back is {}", test);

    println!("Initial bytes:");
    print_bytes(&test);

    let permuted_test = initial_permutation(&test);
    println!("Bytes after permutation:");
    print_bytes(&permuted_test);

    let inverted_test = inverted_permutation(&permuted_test);
    println!("Bytes after inversion:");
    print_bytes(&inverted_test);
}
