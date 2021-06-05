use rand::{thread_rng, Rng};

pub fn random_bytes(length: usize) -> Vec<u8> {
    (0..length).map(|_| thread_rng().gen::<u8>()).collect()
}

pub fn random_in_range(lower: usize, upper: usize) -> usize {
    thread_rng().gen_range(lower..upper)
}

pub fn coin_flip() -> bool {
    thread_rng().gen()
}
