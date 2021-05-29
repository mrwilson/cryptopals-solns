fn profile_for<T: AsRef<[u8]>>(email: T) -> Vec<u8> {
    let mut output = "email=".as_bytes().to_vec();

    let cleaned: Vec<&u8> = email
        .as_ref()
        .into_iter()
        .filter(|c| !(**c == b'&' || **c == b'='))
        .collect();

    output.extend(cleaned);
    output.extend_from_slice("&uid=10&role=user".as_bytes());
    output
}

#[cfg(test)]
mod test {

    use crate::set1::aes;
    use crate::set2::cut_and_paste::profile_for;

    static KEY: [u8; 16] = [
        162, 16, 247, 214, 196, 106, 167, 142, 100, 136, 17, 82, 127, 118, 107, 212,
    ];

    fn oracle<T: AsRef<[u8]>>(input: T) -> Vec<u8> {
        aes::ecb::encrypt(&KEY.to_vec(), profile_for(input))
    }

    fn decrypt<T: AsRef<[u8]>>(input: T) -> Vec<u8> {
        aes::ecb::decrypt(&KEY, input)
    }

    #[test]
    fn break_oracle() {
        // Push "user" off the end so we can remove the block
        let input1 = oracle("a".repeat(13)).into_iter().take(2 * 16);

        // Create ciphertext with a block that starts with "admin"
        let input2 = oracle("aaaaaaaaaaadmin           ");

        // Stitch the blocks together
        let mut synthesised = Vec::new();
        synthesised.extend(input1.clone());
        synthesised.extend(&input2[16..=31]);

        let output = String::from_utf8(decrypt(synthesised)).unwrap();

        assert!(output.contains("&role=admin"));
    }

    #[test]
    fn profile_for_email() {
        let output = profile_for("foo@bar.com");

        assert_eq!(
            output,
            "email=foo@bar.com&uid=10&role=user".as_bytes().to_vec()
        );
    }

    #[test]
    fn profile_for_email_remove_special_chars() {
        let output = profile_for("foo@bar.com&role=admin");

        assert_eq!(
            output,
            "email=foo@bar.comroleadmin&uid=10&role=user"
                .as_bytes()
                .to_vec()
        );
    }
}
