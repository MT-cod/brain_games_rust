pub mod brain_prime {
    use std::{io, process};
    use std::io::BufRead;
    use rand::Rng;
    use inline_colorization::*;#[allow(unused_imports)]
    use crate::common_funcs::common_funcs::get_rand_num;

    pub fn brain_prime() -> (String, String) {
        let random_num = get_rand_num(100);
        let right_answer: String = is_prime(random_num);

        println!("Answer \"yes\" if given number is prime. Otherwise answer \"no\".");
        println!("{color_bright_white}{style_bold}Question: {} ?{style_reset}{color_reset}", random_num);
        println!("Your answer:");
        let input = io::stdin().lock().lines().next().unwrap().unwrap().parse::<String>();
        match input {
            Ok(input) => (right_answer, input),
            Err(_) => process::exit(1),
        }
    }

    pub fn is_prime(num: u32) -> String {
        if num < 2 {
            return "no".to_string();
        }
        let mut i: u32 = 2;
        while i <= num / 2 {
            if num % i == 0 {
                return "no".to_string();
            }
            i += 1;
        }
        return "yes".to_string();
    }
}
