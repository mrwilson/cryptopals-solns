use std::collections::HashMap;

pub fn parse(input: &str) -> HashMap<&str, &str> {
    input
        .split("&")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|entry| {
            let pair: Vec<_> = entry.split("=").collect();
            (pair[0], pair[1])
        })
        .collect::<HashMap<&str, &str>>()
}

#[cfg(test)]
mod test {
    use crate::set2::kv::parse;

    #[test]
    fn parse_key_values() {
        let input = "foo=bar&baz=qux";

        let output = parse(input);

        assert_eq!(output.get("foo").unwrap(), &"bar");
        assert_eq!(output.get("baz").unwrap(), &"qux");
    }
}
