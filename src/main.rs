mod games{
    pub mod brain_calc;
}

use std::io::{self, BufRead};
use std::process;
use crate::games::brain_calc::brain_calc::brain_calc;

fn main() {
    #[derive(Debug)]
    pub struct Game {
        name: String,
        game_num: u8,
        player_is_win: bool,
    }

    impl Game {
        fn greeting() -> Game {
            println!("Welcome to the Brain games!\nMay I have your name?");
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
                Ok(game_num) => Game { name, game_num, player_is_win: false },
                Err(_) => {
                    println!("Illegal game number! Bye!");
                    process::exit(1);
                },
            }
        }

        fn run_game_by_number(self) -> Game  {
            let res: bool = match self.game_num {
                2 => brain_calc(),
                _ => {
                    println!("Start game #{}", &self.game_num);
                    process::exit(1);
                },
            };
            Game {
                name: self.name,
                game_num: self.game_num,
                player_is_win: res,
            }
        }
    }

    let g1 = Game::greeting();
    let g2 = g1.run_game_by_number();
    println!("{:?}", g2);
}
