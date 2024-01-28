mod games{
    pub mod brain_calc;
    pub mod brain_even;
}

use std::io::{self, BufRead};
use std::process;
use crate::games::brain_calc::brain_calc::brain_calc;
use crate::games::brain_even::brain_even::brain_even;

fn main() {
    #[derive(Debug)]
    pub struct Game {
        name: String,
        game_num: u8,
        right_answer: String,
        player_answer: String,
    }

    impl Game {
        fn init() -> Game {
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
                Ok(game_num) => Game {
                    name,
                    game_num,
                    right_answer: "".to_string(),
                    player_answer: "".to_string(),
                },
                Err(_) => {
                    println!("Illegal game number! Bye!");
                    process::exit(1);
                },
            }
        }

        fn run(self) {
            for _i in 1..=3 {
                let game = Game::run_game_by_number(&self.name, self.game_num);
                if game.right_answer == game.player_answer {
                    println!("Correct!");
                } else {
                    println!("\"{}\" is wrong answer ;(. Correct answer was \"{}\".", game.player_answer, game.right_answer);
                    println!("Let's try again, {}!", game.name);
                    break
                }
                if _i == 3 {
                    println!("Congratulations, {}!", game.name);
                }
            }
        }

        fn run_game_by_number(name: &String, game_num: u8) -> Game  {
            let res: (String, String) = match game_num {
                1 => brain_even(),
                2 => brain_calc(),
                _ => {
                    println!("Start game #{}", game_num);
                    process::exit(1);
                },
            };
            Game {
                name: String::from(name),
                game_num,
                right_answer: res.0,
                player_answer: res.1,
            }
        }
    }

    Game::init().run();
}
