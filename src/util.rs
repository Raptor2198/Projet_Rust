use rand::Rng;

pub fn generate_secret_number() -> u32 {
    rand::thread_rng().gen_range(1..=100)
}
