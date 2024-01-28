pub mod common_funcs {
    use rand::Rng;

    pub fn get_rand_num() -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=100)
    }
}
