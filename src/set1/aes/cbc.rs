use crate::set1::aes;
use crate::set1::fixed_xor::fixed_xor;
use crate::set2::pkcs7;

pub fn decrypt<T: AsRef<[u8]>, U: AsRef<[u8]>, V: AsRef<[u8]>>(
    key: T,
    cipher_text: U,
    iv: V,
) -> Vec<u8> {
    let blocks: Vec<&[u8]> = cipher_text.as_ref().chunks(16).collect();

    let mut previous_input: Vec<u8> = iv.as_ref().to_vec();
    let mut output = Vec::new();

    for block in blocks {
        let current_input = block.clone().to_vec();

        let decrypted = aes::ecb::decrypt(key.as_ref(), block);
        let plaintext = fixed_xor(previous_input.clone(), decrypted.clone());

        output.extend(plaintext.clone());

        previous_input = current_input;
    }

    pkcs7::unpad(output).unwrap()
}

pub fn encrypt<T: AsRef<[u8]>, U: AsRef<[u8]>, V: AsRef<[u8]>>(
    key: T,
    plaintext: U,
    iv: V,
) -> Vec<u8> {
    let padded_plaintext = pkcs7::pad(plaintext.as_ref(), 16);

    let blocks: Vec<&[u8]> = padded_plaintext.chunks(16).collect();

    let mut previous_input: Vec<u8> = iv.as_ref().to_vec();
    let mut output = Vec::new();

    for block in blocks {
        let encrypted = aes::ecb::encrypt(
            key.as_ref(),
            fixed_xor(&previous_input, &block.clone().to_vec()),
        );

        output.extend(encrypted.clone());
        previous_input = encrypted.clone();
    }

    output
}

#[cfg(test)]
mod test {
    use crate::set1::aes;
    use crate::set1::base64::from_base64;
    use crate::set1::io::read_file;

    #[test]
    fn decrypt_cbc_test_file() {
        let key = "YELLOW SUBMARINE";
        let iv = b"\x00".repeat(key.len());

        let input: Vec<u8> = read_file("inputs/2_2.txt")
            .into_iter()
            .filter(|c| *c != b'\n')
            .collect();

        let output = aes::cbc::decrypt(key, from_base64(input), iv);

        let as_text = String::from_utf8(output).unwrap();

        assert!(as_text.starts_with("I'm back and I'm ringin' the bell"));
    }

    #[test]
    fn reversibility_cbc() {
        let key = "YELLOW SUBMARINE";
        let input = "Penny Lane, is in my ear, and in my eye.";
        let iv = b"\x00".repeat(key.len());

        let output = aes::cbc::decrypt(key, aes::cbc::encrypt(key, &input, &iv), iv.clone());

        assert_eq!(input.as_bytes(), output)
    }
}
