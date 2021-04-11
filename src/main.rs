mod set1;

use crate::set1::fixed_xor::*;
use crate::set1::hex_to_base64::*;

fn main() {
    println!(
        "{}",
        fixed_xor(
            String::from("1c0111001f010100061a024b53535009181c"),
            String::from("686974207468652062756c6c277320657965")
        )
    );
}
