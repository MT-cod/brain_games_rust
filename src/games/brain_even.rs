pub mod brain_even {
    use std::{io, process};
    use std::io::BufRead;
    use rand::Rng;

    pub fn brain_even() -> (String, String) {

        let random_num = get_rand_num();
        let right_answer = check_even(random_num);

        println!("Answer \"yes\" if the number \"{}\" is even, otherwise answer \"no\".", random_num);
        println!("Your answer:");
        let input = io::stdin().lock().lines().next().unwrap().unwrap().parse::<String>();
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

    pub fn check_even(num: u32) -> String {
        if num % 2 == 0 {
            "yes".to_string()
        } else {
            "no".to_string()
        }
    }
}