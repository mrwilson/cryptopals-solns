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
    use crate::set2::cut_and_paste::profile_for;

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
