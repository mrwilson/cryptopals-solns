mod set1;

use crate::set1::single_byte_xor::*;

fn main() {
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    for a in 0u8..127 {
        println!("{}", single_byte_xor(a as char, input));
    }
}
