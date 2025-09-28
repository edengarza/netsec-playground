pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// Initial Permutation as defined by FIPS 46-3
const IP: [u8; 64] = 
[58, 50, 42, 34, 26, 18, 10, 2,
 60, 52, 44, 36, 28, 20, 12, 4,
 62, 54, 46, 38, 30, 22, 14, 6,
 64, 56, 48, 40, 32, 24, 16, 8,
 57, 49, 41, 33, 25, 17, 9, 1,
 59, 51, 43, 35, 27, 19, 11, 3,
 61, 53, 45, 37, 29, 21, 13, 5,
 63, 55, 47, 39, 31, 23, 15, 7];

// Inverse to Initial Permutation as defined by FIPS 46-3
const IIP: [u8; 64] = 
[40, 8, 48, 16, 56, 24, 64, 32,
 39, 7, 47, 15, 55, 23, 63, 31,
 38, 6, 46, 14, 54, 22, 62, 30,
 37, 5, 45, 13, 53, 21, 61, 29,
 36, 4, 44, 12, 52, 20, 60, 28,
 35, 3, 43, 11, 51, 19, 59, 27,
 34, 2, 42, 10, 50, 18, 58, 26,
 33, 1, 41, 9, 59, 17, 57, 25];

// base permutation function
fn permute_block(block: &u64, permutation_vector: &[u8; 64]) -> u64 {
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

// function to compute the initial permutation
pub fn initial_permutation(input_block: &u64) -> u64 {
    permute_block(&input_block, &IP)
}

// function to computer inverse permutation
pub fn inverted_permutation(preoutput_block: &u64) -> u64 {
    permute_block(&preoutput_block, &IIP)
}
