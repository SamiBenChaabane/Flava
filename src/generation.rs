use rand::{distributions::Alphanumeric, Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub fn password_gen(password_length: &usize) -> String {
    let password: String = ChaCha8Rng::from_entropy()
        .sample_iter(&Alphanumeric)
        .take(*password_length)
        .map(char::from)
        .collect();
    password
}
