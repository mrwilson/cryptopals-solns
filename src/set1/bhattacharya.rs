use std::collections::HashMap;

pub fn bhattacharya(one: HashMap<u8, f32>, two: HashMap<u8, f32>) -> f32 {
    let mut universe: Vec<&u8> = one
        .keys()
        .into_iter()
        .chain(two.keys().into_iter())
        .collect::<Vec<&u8>>();

    universe.sort();
    universe.dedup();

    return universe
        .into_iter()
        .map(|key| match (one.get(&key), two.get(&key)) {
            (Some(f), Some(g)) => (f * g).sqrt(),
            _ => 0f32,
        })
        .sum();
}

#[cfg(test)]
mod test {
    use crate::set1::bhattacharya::bhattacharya;
    use std::collections::HashMap;

    #[test]
    fn identical_inputs_have_coefficient_one() {
        let input: HashMap<u8, f32> = "abcde".chars().map(|char| (char as u8, 0.2f32)).collect();

        assert_eq!(bhattacharya(input.clone(), input.clone()), 1.0);
    }

    #[test]
    fn no_overlap_has_coefficient_zero() {
        let input: HashMap<u8, f32> = [(b'a', 1f32)].iter().cloned().collect();

        let other_input: HashMap<u8, f32> = [(b'b', 1f32)].iter().cloned().collect();

        assert_eq!(bhattacharya(input, other_input), 0.0);
    }
}
