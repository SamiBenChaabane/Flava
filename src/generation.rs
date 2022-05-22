use rand::distributions::Alphanumeric;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
pub fn password_gen() -> String {
    let password: String = ChaCha8Rng::from_entropy()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();
    password
}
