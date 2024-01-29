pub mod common_funcs {
    use rand::Rng;

    pub fn get_rand_num(end_of_range: u32) -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=end_of_range)
    }
}
