// DES according to FIPS 46-3 spec
// not the most secure, but simple to implement

// Initial Permutation vector
const IP: [u8; 64] =
[58, 50, 42, 34, 26, 18, 10, 2,
 60, 52, 44, 36, 28, 20, 12, 4,
 62, 54, 46, 38, 30, 22, 14, 6,
 64, 56, 48, 40, 32, 24, 16, 8,
 57, 49, 41, 33, 25, 17, 9, 1,
 59, 51, 43, 35, 27, 19, 11, 3,
 61, 53, 45, 37, 29, 21, 13, 5,
 63, 55, 47, 39, 31, 23, 15, 7];

// Inverse to Initial Permutation vector
const IIP: [u8; 64] =
[40, 8, 48, 16, 56, 24, 64, 32,
 39, 7, 47, 15, 55, 23, 63, 31,
 38, 6, 46, 14, 54, 22, 62, 30,
 37, 5, 45, 13, 53, 21, 61, 29,
 36, 4, 44, 12, 52, 20, 60, 28,
 35, 3, 43, 11, 51, 19, 59, 27,
 34, 2, 42, 10, 50, 18, 58, 26,
 33, 1, 41, 9, 49, 17, 57, 25];

// Permuted Choice 1 for component C
const PC1C: [u8; 28] =
[57, 49, 41, 33, 25, 17, 9,
 1, 58, 50, 42, 34, 26, 18,
 10, 2, 59, 51, 43, 35, 27,
 19, 11, 3, 60, 52, 44, 36];

// Permuted Choice 1 for component D
const PC1D: [u8; 28] =
[63, 55, 47, 39, 31, 23, 15,
 7, 62, 54, 46, 38, 30, 22,
 14, 6, 61, 53, 45, 37, 29,
 21, 13, 5, 28, 20, 12, 4];

// Permuted Choice 2
const PC2: [u8; 48] =
[14, 17, 11, 24, 1, 5,
 3, 28, 15, 6, 21, 10,
 23, 19, 12, 4, 26, 8,
 16, 7, 27, 20, 13, 2,
 41, 52, 31, 37, 47, 55,
 30, 40, 51, 45, 33, 48,
 44, 49, 39, 56, 34, 53,
 46, 42, 50, 36, 29, 32];

// Left Shift Schedule
const LSS: [u8; 16] =
[1, 1, 2, 2,
 2, 2, 2, 2,
 1, 2, 2, 2,
 2, 2, 2, 1];

// sorry, got too annoyed manually copying, thanks LLMs uwu
// Expansion function to convert 32 bits to 48
const E_BIT_SELECTION_TABLE: [u8; 48] = [
    32,  1,  2,  3,  4,  5,
     4,  5,  6,  7,  8,  9,
     8,  9, 10, 11, 12, 13,
    12, 13, 14, 15, 16, 17,
    16, 17, 18, 19, 20, 21,
    20, 21, 22, 23, 24, 25,
    24, 25, 26, 27, 28, 29,
    28, 29, 30, 31, 32,  1,
];

// Selection functions for converting 6 bit segments to 4 bit ones
const S1: [u8; 64] = [
    14,  4, 13,  1,  2, 15, 11,  8,  3, 10,  6, 12,  5,  9,  0,  7,
     0, 15,  7,  4, 14,  2, 13,  1, 10,  6, 12, 11,  9,  5,  3,  8,
     4,  1, 14,  8, 13,  6,  2, 11, 15, 12,  9,  7,  3, 10,  5,  0,
    15, 12,  8,  2,  4,  9,  1,  7,  5, 11,  3, 14, 10,  0,  6, 13,
];

const S2: [u8; 64] = [
    15,  1,  8, 14,  6, 11,  3,  4,  9,  7,  2, 13, 12,  0,  5, 10,
     3, 13,  4,  7, 15,  2,  8, 14, 12,  0,  1, 10,  6,  9, 11,  5,
     0, 14,  7, 11, 10,  4, 13,  1,  5,  8, 12,  6,  9,  3,  2, 15,
    13,  8, 10,  1,  3, 15,  4,  2, 11,  6,  7, 12,  0,  5, 14,  9,
];

const S3: [u8; 64] = [
    10,  0,  9, 14,  6,  3, 15,  5,  1, 13, 12,  7, 11,  4,  2,  8,
    13,  7,  0,  9,  3,  4,  6, 10,  2,  8,  5, 14, 12, 11, 15,  1,
    13,  6,  4,  9,  8, 15,  3,  0, 11,  1,  2, 12,  5, 10, 14,  7,
     1, 10, 13,  0,  6,  9,  8,  7,  4, 15, 14,  3, 11,  5,  2, 12,
];

const S4: [u8; 64] = [
     7, 13, 14,  3,  0,  6,  9, 10,  1,  2,  8,  5, 11, 12,  4, 15,
    13,  8, 11,  5,  6, 15,  0,  3,  4,  7,  2, 12,  1, 10, 14,  9,
    10,  6,  9,  0, 12, 11,  7, 13, 15,  1,  3, 14,  5,  2,  8,  4,
     3, 15,  0,  6, 10,  1, 13,  8,  9,  4,  5, 11, 12,  7,  2, 14,
];

const S5: [u8; 64] = [
     2, 12,  4,  1,  7, 10, 11,  6,  8,  5,  3, 15, 13,  0, 14,  9,
    14, 11,  2, 12,  4,  7, 13,  1,  5,  0, 15, 10,  3,  9,  8,  6,
     4,  2,  1, 11, 10, 13,  7,  8, 15,  9, 12,  5,  6,  3,  0, 14,
    11,  8, 12,  7,  1, 14,  2, 13,  6, 15,  0,  9, 10,  4,  5,  3,
];

const S6: [u8; 64] = [
    12,  1, 10, 15,  9,  2,  6,  8,  0, 13,  3,  4, 14,  7,  5, 11,
    10, 15,  4,  2,  7, 12,  9,  5,  6,  1, 13, 14,  0, 11,  3,  8,
     9, 14, 15,  5,  2,  8, 12,  3,  7,  0,  4, 10,  1, 13, 11,  6,
     4,  3,  2, 12,  9,  5, 15, 10, 11, 14,  1,  7,  6,  0,  8, 13,
];

const S7: [u8; 64] = [
     4, 11,  2, 14, 15,  0,  8, 13,  3, 12,  9,  7,  5, 10,  6,  1,
    13,  0, 11,  7,  4,  9,  1, 10, 14,  3,  5, 12,  2, 15,  8,  6,
     1,  4, 11, 13, 12,  3,  7, 14, 10, 15,  6,  8,  0,  5,  9,  2,
     6, 11, 13,  8,  1,  4, 10,  7,  9,  5,  0, 15, 14,  2,  3, 12,
];

const S8: [u8; 64] = [
    13,  2,  8,  4,  6, 15, 11,  1, 10,  9,  3, 14,  5,  0, 12,  7,
     1, 15, 13,  8, 10,  3,  7,  4, 12,  5,  6, 11,  0, 14,  9,  2,
     7, 11,  4,  1,  9, 12, 14,  2,  0,  6, 10, 13, 15,  3,  5,  8,
     2,  1, 14,  7,  4, 10,  8, 13, 15, 12,  9,  0,  3,  5,  6, 11,
];

const LOOKUP_TABLES: [[u8; 64]; 8] = [S1, S2, S3, S4, S5, S6, S7, S8];

const CIPHER_PERMUTATION: [u8; 32] = [
    16,  7, 20, 21,
    29, 12, 28, 17,
     1, 15, 23, 26,
     5, 18, 31, 10,
     2,  8, 24, 14,
    32, 27,  3,  9,
    19, 13, 30,  6,
    22, 11,  4, 25,
];

// base permutation function
fn permute_block_64(block: &u64, permutation_vector: &[u8; 64]) -> u64 {
    let mut permuted_block: u64 = 0;

    // one set of operations for each of the 64 bits
    for i in 0..64 {
        // DES numbers the bits  [1, 2, ..., 63, 64]
        // shifts number bits as [63, 62, ..., 1, 0]
        // (which makes sense, shift 0 to get the low order bit)
        // this diff means DES given values need to be -1
        // additionally, since the order is flipped, subtract desired index from max val (64 - 1)

        // determine the shift amount based on the vector
        let bit_index = permutation_vector[i] - 1;

        // isolate the source bit using right shifts and AND
        let bit_value = (block >> (63 - bit_index)) & 1;

        // expand bit value to correct position
        // again, adjust for notation diff
        let bit_value_expanded = bit_value << (63 - i);

        // add the value to the result
        permuted_block |= bit_value_expanded;
    }

    permuted_block
}

// permute a value held in 64 bits into 48 bits
// this is used by expansion function to expand 32 bits to 48
fn permute_block_48(block: &u64, permutation_vector: &[u8; 48]) -> u64 {
    let mut permuted_block: u64 = 0;

    // one iteration for each bit to write
    for i in 0..48 {
        let bit_index = permutation_vector[i] - 1;

        // the values are held in the rightmost bits, so index from there
        let bit_value = (block >> (31 - bit_index)) & 1;
        let bit_value_expanded = bit_value << (47 - i);
        permuted_block |= bit_value_expanded;
    }

    permuted_block

}

// permute a 56 bit value held in 64 bits to 48 bits
// used during the key schedule generation phase
fn permute_56_block_to_48(block: &u64, permutation_vector: &[u8; 48]) -> u64 {
    let mut permuted_block: u64 = 0;

    // one iteration for each bit to write
    for i in 0..48 {
        let bit_index = permutation_vector[i] - 1;

        // the values are held in the rightmost bits, so index from there
        let bit_value = (block >> (55 - bit_index)) & 1;
        let bit_value_expanded = bit_value << (47 - i);
        permuted_block |= bit_value_expanded;
    }

    permuted_block
}

// permute a 32 bit value into another 32 bit value
fn permute_block_32(block: &u32, permutation_vector: &[u8; 32]) -> u32 {
    let mut permuted_block: u32 = 0;

    // one iteration for each bit to write
    for i in 0..32 {
        let bit_index = permutation_vector[i] - 1;
        let bit_value: u32 = ((block >> (31 - bit_index)) & 1).try_into().unwrap();
        let bit_value_expanded = bit_value << (31 - i);
        permuted_block |= bit_value_expanded;
    }

    permuted_block
}

// permute a 64 bit value into 28 bits
fn permute_block_28(block: &u64, permutation_vector: &[u8; 28]) -> u32 {
    let mut permuted_block: u32 = 0;

    // one iteration for each bit to write
    for i in 0..28 {
        let bit_index = permutation_vector[i] - 1;
        let bit_value: u32 = ((block >> (63 - bit_index)) & 1).try_into().unwrap();
        let bit_value_expanded = bit_value << (27 - i);
        permuted_block |= bit_value_expanded;
    }

    permuted_block
}

// function to left shift according to key schedule specs
// specifically, need to take special care to keep values
// in lower 28 bit positions, since stored in u32 type
fn circular_left_shift_28(component: &u32) -> u32 {
    let mask: u32 = 0b00001111111111111111111111111111;
    let mut shifted_component: u32;

    // store the value of the bit in position 28
    let carry_over = (component >> 27) & 1;

    // shift everything by one
    shifted_component = component << 1;

    // reinsert the carryover
    shifted_component |= carry_over;

    // clear any values in 4 high order bits
    shifted_component &= mask;

    shifted_component
}

// function to combine two 28 bit values into a 56 bit one
// the 28 bit values are stored in 32 bits
// the 56 bit value will be stored in 64 bits
fn concatenate_28_bit_blocks(c: &u32, d: &u32) -> u64 {
    let mask: u64 = 0b0000000011111111111111111111111111111111111111111111111111111111;
    let mut combined: u64 = 0;

    // d is already in a good spot, as low order bits
    // c needs to be shifted into position
    let mut moved_c: u64 = 0;
    moved_c |= *c as u64;
    moved_c = moved_c << 28;

    // assemble the components
    combined |= moved_c;
    combined |= *d as u64;

    // clear any values in 8 high order bits
    combined &= mask;

    combined
}

// takes a series of 8 groups of 4 bits and forms a 32 bit value
fn concatenate_4_bit_blocks(blocks: &[u8; 8]) -> u32 {
    let mut combined: u32 = 0;

    // proceed left to right as DES expects
    for i in 0..8 {
        // build from the low order out
        combined |= blocks[i] as u32;

        // handle fencepost for the end
        if i != 7 {
            combined = combined << 4;
        }
    }

    combined
}

// function to expand a 32 bit block to 48 bits to perform cipher function
fn expand_block_48(block: &u32) -> u64 {
    let mask: u64 = 0b0000000011111111111111111111111111111111111111111111111111111111;

    // convert to 64 bits (safe conversion) to use existing function
    let cast_block = *block as u64;

    // use existing function with expansion table
    let mut expanded_block = permute_block_48(&cast_block, &E_BIT_SELECTION_TABLE);

    // clear any values in 8 high order bits
    expanded_block &= mask;

    expanded_block
}

// helper to take 48 bits from XOR into 8 6-bit groups
fn extract_6_bit_groups(block: &u64) -> [u8; 8] {
    let mut bit_groups: [u8; 8] = [0; 8];
    let group_size = 6;
    let mask: u8 = 0b00111111;

    // iterate backwards since DES goes left to right
    for i in (0..8).rev() {
        // perform the needed right shifts
        let current_group = block >> ((7 - i) * group_size);

        // casting truncates to the lowest 8 bits
        let least_significant_byte: u8 = current_group as u8;

        // remove 2 high order bits
        let value = least_significant_byte & mask;

        // assign into the array
        bit_groups[i] = value;
    }

    bit_groups
}

// function to take 6 bits and lookup 4 bit value
// first and final bit determine the row
// middle four bits determine the col
// tables are 4x16, so calc is (row * 16) + col
fn reduce_bit_group(bit_group: &u8, lookup_table: &[u8; 64]) -> u8 {
    // determine the row
    let row_low = bit_group & 1;
    let row_high = (bit_group >> 5) & 1;
    let row = (2 * row_high) + row_low;

    // determine the col
    let col = (bit_group >> 1) & 15;

    // calculate the index in the loopup_table
    let index = (row * 16) + col;

    // get the value and return
    let value = lookup_table[index as usize];

    value
}

// function to break a 64 bit value into two 32 bit values
fn divide_64_bit_block(block: &u64) -> (u32, u32) {
    let right: u32 = *block as u32;
    let left: u32 = (block >> 32) as u32;

    (left, right)
}

// function to combine two 32 bit values into a 64 bit value
fn combine_2_32_bit_blocks(left: &u32, right: &u32) -> u64 {
    let mut output: u64 = 0;

    // add the left side first
    output |= *left as u64;

    // shift bits into position
    output <<= 32;

    // add the right side
    output |= *right as u64;

    output
}

// function to compute the initial permutation
pub fn initial_permutation(input_block: &u64) -> u64 {
    permute_block_64(&input_block, &IP)
}

// function to computer inverse permutation
pub fn inverted_permutation(preoutput_block: &u64) -> u64 {
    permute_block_64(&preoutput_block, &IIP)
}

// function to produce the key schedule based on given key
pub fn generate_key_schedule(key: &u64) -> [u64; 16] {
    // keys in the schedule are 48 bits but that's not a data type
    // instead I'll (attempt) to store it using leading 0s in u64s
    // components c and d are both 32 bits

    let mut key_schedule: [u64; 16] = [0; 16];

    // compute the two components
    let mut c: u32 = permute_block_28(&key, &PC1C);
    let mut d: u32 = permute_block_28(&key, &PC1D);

    // generate each of the 16 keys
    for i in 0..16 {
        // shift according to the schedule
        let shift_amount: u8 = LSS[i];

        for _ in 0..shift_amount {
            c = circular_left_shift_28(&c);
            d = circular_left_shift_28(&d);
        }

        // combine the keys into 56 bits
        let cd: u64 = concatenate_28_bit_blocks(&c, &d);

        // permute to 48 bits
        let key: u64 = permute_56_block_to_48(&cd, &PC2);

        // store in key_schedule array
        key_schedule[i] = key;
    }

    key_schedule
}

// function to encipher a 32 bit block with a 48 bit key
pub fn cipher_function(block: &u32, key: &u64) -> u32 {
    let mut reduced_bit_groups: [u8; 8] = [0; 8];

    // expand the given block to match the key length
    let expanded_block: u64 = expand_block_48(&block);

    // XOR the block and they key
    let block_xor_key: u64 = expanded_block ^ key;

    // extract the 8 sets of 6 bits
    let bit_groups: [u8; 8] = extract_6_bit_groups(&block_xor_key);

    // reduce bit groups and store
    for i in 0..8 {
        reduced_bit_groups[i] = reduce_bit_group(&bit_groups[i], &LOOKUP_TABLES[i]);
    }

    // combine the reduced groups into one number
    let preoutput: u32 = concatenate_4_bit_blocks(&reduced_bit_groups);

    // perform final permutation
    let output: u32 = permute_block_32(&preoutput, &CIPHER_PERMUTATION);

    output
}

// main encryption function
// takes a 64 bit block and a precomputed key schedule
// to produce a 64 bit ciphertext
pub fn encipher_block(block: &u64, key_schedule: &[u64; 16]) -> u64 {
    let mut left: u32;
    let mut right: u32;

    // perform the initial permutation
    let initial_permutation_block = initial_permutation(&block);

    // get the initial left and right components
    (left, right) = divide_64_bit_block(&initial_permutation_block);

    // perform 16 rounds of cipher function operations
    for key in key_schedule.iter() {
        // left prime is simply right prior to any transformations
        let left_prime = right;

        // perform XOR of left with the cipher function on the right block using ith key
        let right_prime = left ^ cipher_function(&right, &key);

        // reassign left and right values
        left = left_prime;
        right = right_prime;
    }

    // combine values back to 64 bit value
    let preoutput_block = combine_2_32_bit_blocks(&right, &left);

    // perform the inverse of the initial permutation
    let output = inverted_permutation(&preoutput_block);

    output
}

// main decryption function
// takes a 64 bit block and a precomputed key schedule
pub fn decipher_block(block: &u64, key_schedule: &[u64; 16]) -> u64 {
    let mut left: u32;
    let mut right: u32;

    // perform the initial permutation
    let initial_permutation_block = initial_permutation(&block);

    // get the initial left and right components
    (left, right) = divide_64_bit_block(&initial_permutation_block);

    // perform 16 rounds of cipher function operations in reverse
    for key in key_schedule.iter().rev() {
        // left prime is simply right prior to any transformations
        let left_prime = right;

        // perform XOR of left with the cipher function on the right block using ith key
        let right_prime = left ^ cipher_function(&right, &key);

        // reassign left and right values
        left = left_prime;
        right = right_prime;
    }

    // combine values back to 64 bit value
    let preoutput_block = combine_2_32_bit_blocks(&right, &left);

    // perform the inverse of the initial permutation
    let output = inverted_permutation(&preoutput_block);

    output
}

// --- Tests ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circular_left_shift_1() {
        let value: u32 = 1;
        let shifted_value: u32 = circular_left_shift_28(&value);
        assert_eq!(shifted_value, 2);
    }

    #[test]
    fn circular_left_shift_2() {
        let value: u32 = 2;
        let shifted_value: u32 = circular_left_shift_28(&value);
        assert_eq!(shifted_value, 4);
    }

    #[test]
    fn circular_left_shift_27() {
        let value: u32 = 1 << 27;
        let shifted_value: u32 = circular_left_shift_28(&value);
        assert_eq!(shifted_value, 1);
    }

    #[test]
    fn concatenate_28_bit_blocks_low() {
        let c: u32 = 1;
        let d: u32 = 1;
        let concat_value: u64 = concatenate_28_bit_blocks(&c, &d);
        let expected: u64 = 0b0000000000000000000000000000000000010000000000000000000000000001;
        assert_eq!(concat_value, expected);
    }

    #[test]
    fn concatenate_28_bit_blocks_high() {
        let c: u32 = 1 << 27;
        let d: u32 = 1 << 27;
        let concat_value: u64 = concatenate_28_bit_blocks(&c, &d);
        let expected: u64 = 0b0000000010000000000000000000000000001000000000000000000000000000;
        assert_eq!(concat_value, expected);
    }

    #[test]
    fn concatenate_28_bit_blocks_max() {
        let c: u32 = u32::MAX;
        let d: u32 = u32::MAX;
        let concat_value: u64 = concatenate_28_bit_blocks(&c, &d);
        let expected: u64 = 0b0000000011111111111111111111111111111111111111111111111111111111;
        assert_eq!(concat_value, expected);
    }

    #[test]
    fn divide_64_bit_block_1() {
        let test_value: u64 = 0x0000000100000001;
        let expected: u32 = 0x00000001;
        let (left, right) = divide_64_bit_block(&test_value);
        assert_eq!(left, expected);
        assert_eq!(right, expected);
    }

    #[test]
    fn divide_64_bit_block_2() {
        let test_value: u64 = 0x0000000110000000;
        let expected_left: u32 = 0x00000001;
        let expected_right: u32 = 0x10000000;
        let (left, right) = divide_64_bit_block(&test_value);
        assert_eq!(left, expected_left);
        assert_eq!(right, expected_right);
    }

    #[test]
    fn divide_64_bit_block_3() {
        let test_value: u64 = 0xdeadbeefdeadbeef;
        let expected: u32 = 0xdeadbeef;
        let (left, right) = divide_64_bit_block(&test_value);
        assert_eq!(left, expected);
        assert_eq!(right, expected);
    }

    #[test]
    fn combine_2_32_bit_blocks_1() {
        let left: u32 = 0x00000001;
        let expected: u64 = 0x0000000100000001;
        let combined: u64 = combine_2_32_bit_blocks(&left, &left);
        assert_eq!(combined, expected);
    }

    #[test]
    fn combine_2_32_bit_blocks_2() {
        let left: u32 = 0x00000001;
        let right: u32 = 0x10000000;
        let expected: u64 = 0x0000000110000000;
        let combined: u64 = combine_2_32_bit_blocks(&left, &right);
        assert_eq!(combined, expected);
    }

    #[test]
    fn combine_2_32_bit_blocks_3() {
        let left: u32 = 0xdeadbeef;
        let right: u32 = 0xdeadbeef;
        let expected: u64 = 0xdeadbeefdeadbeef;
        let combined: u64 = combine_2_32_bit_blocks(&left, &right);
        assert_eq!(combined, expected);
    }
}
