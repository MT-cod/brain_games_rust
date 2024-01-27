pub mod brain_calc {
    use std::{io, process};
    use std::io::BufRead;
    use rand::Rng;

    pub fn brain_calc() -> (u32, u32) {
        println!("What is the result of the expression?");
        let random_num1 = get_rand_num();
        let random_num2 = get_rand_num();
        let (oper_num, oper_str): (u8, char) = get_rand_calc_operation();
        let right_answer = calc_operation(random_num1, random_num2, oper_num);

        println!("Question: {} {} {} ?", random_num1, oper_str, random_num2);
        println!("Your answer:");
        let input = io::stdin().lock().lines().next().unwrap().unwrap().parse::<u32>();
        match input {
            Ok(input) => (right_answer, input),
            Err(_) => {
                println!("Illegal answer! Bye!");
                process::exit(1);
            },
        }
    }

    pub fn get_rand_num() -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=100)
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