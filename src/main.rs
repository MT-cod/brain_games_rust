mod games {
    pub mod brain_calc;
    pub mod brain_even;
    pub mod brain_gcd;
    pub mod brain_prime;
    pub mod brain_progression;
}
pub mod common_funcs;

#[allow(unused_imports)]
use crate::games::brain_calc::brain_calc::brain_calc;
use crate::games::brain_even::brain_even::brain_even;
use crate::games::brain_gcd::brain_gcd::brain_gcd;
use crate::games::brain_prime::brain_prime::brain_prime;
use crate::games::brain_progression::brain_progression::brain_progression;
use inline_colorization::*;
use std::io::{self, BufRead};
use std::process;

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
            println!("{color_cyan}{style_bold}Welcome to the Brain games!{style_reset}{color_reset}\nMay I have your name?");
            let name = io::stdin().lock().lines().next().unwrap().unwrap();

            println!("Hello, {}!", name);
            println!("{style_bold}{color_yellow}\nList of available games:");
            println!("{color_bright_yellow}-----------------------");
            println!("1. Brain even");
            println!("2. Brain calc");
            println!("3. Brain GCD");
            println!("4. Brain progression");
            println!("5. Brain prime");
            println!("-----------------------{style_reset}{color_reset}");
            println!("Enter game number please:");

            match io::stdin()
                .lock()
                .lines()
                .next()
                .unwrap()
                .unwrap()
                .parse::<u8>()
            {
                Ok(game_num) => Game {
                    name,
                    game_num,
                    right_answer: "".to_string(),
                    player_answer: "".to_string(),
                },
                Err(_) => {
                    println!("{color_bright_red}Illegal input! Bye!{color_reset}");
                    process::exit(1);
                }
            }
        }

        fn run(self) {
            for _i in 1..=3 {
                let game = Game::run_game_by_number(&self.name, self.game_num);
                if game.right_answer == game.player_answer {
                    println!("{color_cyan}Correct!{color_reset}");
                } else {
                    println!("{color_bright_red}\"{}\" is wrong answer ;(. Correct answer was \"{}\".{color_reset}", game.player_answer, game.right_answer);
                    println!("Let's try again, {}!", game.name);
                    break;
                }
                if _i == 3 {
                    println!("{color_bright_cyan}{style_bold}Congratulations, {}!{color_reset}{style_reset}", game.name);
                }
            }
        }

        fn run_game_by_number(name: &String, game_num: u8) -> Game {
            let res: (String, String) = match game_num {
                1 => brain_even(),
                2 => brain_calc(),
                3 => brain_gcd(),
                4 => brain_progression(),
                5 => brain_prime(),
                _ => {
                    println!(
                        "{color_bright_red}Illegal game number \"{}\"! Bye!{color_reset}",
                        game_num
                    );
                    process::exit(1);
                }
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
