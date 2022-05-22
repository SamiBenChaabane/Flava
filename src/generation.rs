use rand::distributions::Alphanumeric;
use rand::Rng;

pub fn password_gen() -> String {
    let password: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();
    password
}
