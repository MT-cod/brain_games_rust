pub mod brain_progression {
    use std::{io, process};
    use std::io::BufRead;
    use inline_colorization::*;#[allow(unused_imports)]
    use crate::common_funcs::common_funcs::get_rand_num;

    pub fn brain_progression() -> (String, String) {
        let random_start_num = get_rand_num(100);
        let random_progression_pos_num = get_rand_num(10);
        let random_progressor_val = get_rand_num(10);
        let right_answer = random_start_num + random_progressor_val * (random_progression_pos_num - 1);
        let mut progress_line: Vec<String> = vec![];
        for i in 0..10 {
            if i == random_progression_pos_num - 1 {
                progress_line.push("..".parse().unwrap());
            } else {
                progress_line.push((random_start_num + random_progressor_val * i).to_string());
            }
        }
        let print_line: String = progress_line.join(" ");

        println!("{color_bright_white}{style_bold}What number is missing in the progression?{style_reset}{color_reset}");
        println!("{color_bright_white}{style_bold}Question: \"{}\"{style_reset}{color_reset}", print_line);
        println!("Your answer:");
        let input = io::stdin().lock().lines().next().unwrap().unwrap().parse::<String>();
        match input {
            Ok(input) => (right_answer.to_string(), input),
            Err(_) => process::exit(1),
        }
    }
}