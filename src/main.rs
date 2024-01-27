use std::io::{self, BufRead};
use std::process;

fn main() {
    #[derive(Debug)]
    pub struct Game {
        name: String,
        game_num: u8,
        games_counter: u8,
    }

    impl Game {
        fn greeting() -> Game {
            println!("Welcome to the Brain Games!\nMay I have your name?");
            let name = io::stdin().lock().lines().next().unwrap().unwrap();

            println!("Hello, {}!", name);
            println!("\nList of available games:");
            println!("-----------------------");
            println!("1. Brain even");
            println!("2. Brain calc");
            println!("3. Brain GCD");
            println!("4. Brain progression");
            println!("5. Brain prime");
            println!("-----------------------");
            println!("Enter game number please:");

            match io::stdin().lock().lines().next().unwrap().unwrap().parse::<u8>() {
                Ok(game_num) => Game { name, game_num, games_counter: 0 },
                Err(_) => {
                    println!("Illegal game number! Bye!");
                    process::exit(1);
                },
            }
        }
    }

    let g = Game::greeting();
    println!("{:?}", g);
}
