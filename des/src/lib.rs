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

// permute a 56 bit value into 48 bits
fn permute_block_48(block: &u64, permutation_vector: &[u8; 48]) -> u64 {
    let mut permuted_block: u64 = 0;

    // one iteration for each bit to write
    for i in 0..48 {
        let bit_index = permutation_vector[i] - 1;
        let bit_value = (block >> (63 - bit_index)) & 1;
        let bit_value_expanded = bit_value << (47 - i);
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
    let mut c = permute_block_28(&key, &PC1C);
    let mut d = permute_block_28(&key, &PC1D);

    // generate each of the 16 keys
    for i in 0..16 {
        // shift according to the schedule
        let shift_amount = LSS[i];

        for _ in 0..shift_amount {
            c = circular_left_shift_28(&c);
            d = circular_left_shift_28(&d);
        }

        // combine the keys into 56 bits
        let cd = concatenate_28_bit_blocks(&c, &d);

        // permute to 48 bits
        let key = permute_block_48(&cd, &PC2);

        // store in key_schedule array
        key_schedule[i] = key;
    }

    key_schedule
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
}
