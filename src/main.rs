use std::io::{self, BufRead};

fn main() {
    #[derive(Debug)]
    pub struct Game {
        name: String,
        game_num: u8,
        games_counter: u8,
    }

    impl Game {
        fn greeting() -> Self {
            println!("Welcome to the Brain Games!\nMay I have your name?");
            let name = io::stdin().lock().lines().next().unwrap().unwrap();

            println!("Hello, {}!", name);
            println!("\nList of available games:");
            println!("1. Brain even");
            println!("2. Brain calc");
            println!("3. Brain GCD");
            println!("4. Brain progression");
            println!("5. Brain prime");
            println!("Enter game number please:");
            let game_num = io::stdin().lock().lines().next().unwrap().unwrap().parse::<u8>().unwrap();

            Game {
                name: name,
                game_num: game_num,
                games_counter: 0,
            }
        }
    }

    let g = Game::greeting();
    println!("{:?}", g);
}
