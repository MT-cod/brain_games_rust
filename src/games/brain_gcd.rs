pub mod brain_gcd {
    #[allow(unused_imports)]
    use crate::common_funcs::common_funcs::get_rand_num;
    use inline_colorization::*;
    use std::io::BufRead;
    use std::{io, process};

    pub fn brain_gcd() -> (String, String) {
        let random_num1 = get_rand_num(100);
        let random_num2 = get_rand_num(100);
        let right_answer = get_gcd(random_num1, random_num2);

        println!("{color_bright_white}{style_bold}Find the greatest common divisor of given numbers.{style_reset}{color_reset}");
        println!(
            "{color_bright_white}{style_bold}Question: \"{}\" and \"{}\"{style_reset}{color_reset}",
            random_num1, random_num2
        );
        println!("Your answer:");
        let input = io::stdin()
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .parse::<String>();
        match input {
            Ok(input) => (right_answer.to_string(), input),
            Err(_) => process::exit(1),
        }
    }

    fn get_gcd(mut num1: u32, mut num2: u32) -> u32 {
        while num1 % num2 != 0 {
            let trans_num1 = num1.clone();
            let trans_num2 = num2.clone();
            num1 = trans_num2;
            num2 = trans_num1 % trans_num2;
        }
        num2
    }
}
