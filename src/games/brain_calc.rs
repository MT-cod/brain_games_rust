pub mod brain_calc {
    use std::{io, process};
    use std::io::BufRead;
    use rand::Rng;
    use inline_colorization::*;#[allow(unused_imports)]
    use crate::common_funcs::common_funcs::get_rand_num;

    pub fn brain_calc() -> (String, String) {
        let random_num1 = get_rand_num(100);
        let random_num2 = get_rand_num(100);
        let (oper_num, oper_str): (u8, char) = get_rand_calc_operation();
        let right_answer = calc_operation(random_num1, random_num2, oper_num);

        println!("What is the result of the expression?");
        println!("{color_bright_white}{style_bold}Question: {} {} {} ?{style_reset}{color_reset}", random_num1, oper_str, random_num2);
        println!("Your answer:");
        let input = io::stdin().lock().lines().next().unwrap().unwrap().parse::<String>();
        match input {
            Ok(input) => (right_answer.to_string(), input),
            Err(_) => process::exit(1),
        }
    }

    pub fn get_rand_calc_operation() -> (u8, char) {
        let mut rng = rand::thread_rng();
        let oper_num: u8 = rng.gen_range(1..=3);

        match oper_num {
            1 => (1, '+'),
            2 => (2, '-'),
            3 => (3, '*'),
            _ => process::exit(1)
        }
    }

    pub fn calc_operation(num1: u32, num2: u32, oper_num: u8) -> u32 {
        match oper_num {
            1 => num1 + num2,
            2 => num1 - num2,
            3 => num1 * num2,
            _ => process::exit(1)
        }
    }
}
