pub mod brain_even {
    use std::{io, process};
    use std::io::BufRead;
    use inline_colorization::*;#[allow(unused_imports)]
    use crate::common_funcs::common_funcs::get_rand_num;

    pub fn brain_even() -> (String, String) {

        let random_num = get_rand_num(100);
        let right_answer = check_even(random_num);

        println!("{color_bright_white}{style_bold}Answer \"yes\" if the number \"{}\" is even, otherwise answer \"no\".{style_reset}{color_reset}", random_num);
        println!("Your answer:");
        let input = io::stdin().lock().lines().next().unwrap().unwrap().parse::<String>();
        match input {
            Ok(input) => (right_answer, input),
            Err(_) => process::exit(1),
        }
    }

    pub fn check_even(num: u32) -> String {
        if num % 2 == 0 {
            "yes".to_string()
        } else {
            "no".to_string()
        }
    }
}