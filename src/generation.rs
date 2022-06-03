use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub fn password_gen(password_length: &usize) -> String {
    //This code generates special chars, letters and numbers.
    let mut rng = ChaCha8Rng::from_entropy();
    (0..*password_length)
        .map(|_| (0x21u8 + (rng.gen_range(0.1..0.97) * 96.0) as u8) as char)
        .collect()
}
