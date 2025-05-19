use std::cmp::Ordering;
// use std::fs;
use std::io::{self, Write};
// use std::path::Path;
use std::time::Instant;

fn start_game() {
    println!("\n-----------------------------");
    println!("    NUMBER_GUESSING_GAME!    ");
    println!("-----------------------------");
    println!("SELECT * DIFFICULTY FROM GAME");
    println!("- EZ\n- MID\n- TUFF\n");

    let stdin = io::stdin();
    let mut diff = String::with_capacity(4);

    print!("> ");
    io::stdout().flush().unwrap();
    stdin.read_line(&mut diff).unwrap();
    let diff = diff.to_lowercase();
    let diff = diff.trim();

    let mut chances = match diff {
        "ez" => 10,
        "mid" => 5,
        "tuff" => 3,
        _ => {
            println!("Invalid difficulty.");
            std::process::exit(0);
        }
    };

    println!("\nDifficulty selected: {}, chance count: {}", diff, chances);
    println!("Guess a number between 1..=100, good luck!!!\n");
    let time = Instant::now();
    let correct_num = fastrand::u8(1..=100);
    let mut answer = String::with_capacity(3);
    let mut attempts = 0;
    let mut won = false;
    while chances > 0 {
        print!("> ");
        io::stdout().flush().unwrap();
        answer.clear();
        stdin.read_line(&mut answer).unwrap();
        if let Ok(n) = answer.trim().parse::<u8>() {
            match n.cmp(&correct_num) {
                Ordering::Less => {
                    println!("{} is lower than the correct answer!", n);
                    chances -= 1;
                    attempts += 1;
                }
                Ordering::Equal => {
                    attempts += 1;
                    won = true;
                    println!("\nCorrect guess, you win!!!");
                    println!(
                        "attempts: {}, time taken: {} seconds",
                        attempts,
                        time.elapsed().as_secs()
                    );
                    break;
                }
                Ordering::Greater => {
                    println!("{} is higher than the correct answer!", n);
                    chances -= 1;
                    attempts += 1;
                }
            }
        } else {
            println!("Uh oh, you just wasted a chance by inputting an invalid number.");
            chances -= 1;
            attempts += 1;
        }
    }

    if !won {
        println!("\nYou lost!");
        println!("elapsed time: {} seconds", time.elapsed().as_secs());
    }

    println!("\nPlay again? (y/n)");
    let mut play_again = String::with_capacity(1);
    print!("> ");
    io::stdout().flush().unwrap();
    stdin.read_line(&mut play_again).unwrap();
    let play_again = play_again.to_lowercase();
    let play_again = play_again.trim();

    if play_again == "y" {
        start_game();
    } else {
        std::process::exit(0);
    }
}

// fn parse_highscore(highscore: &str) -> [(i64, i64); 3] {
//     let splitted: Vec<&str> = highscore.trim().split(",").collect();

//     if splitted.len() != 3 {
//         fs::write("./highscore.txt", "0-0,0-0,0-0").unwrap();
//         parse_highscore(&fs::read_to_string("./highscore.txt").unwrap());
//     }

//     let mut result = [(0, 0); 3];

//     for (i, pair_str) in splitted.iter().enumerate() {
//         let parts: Vec<&str> = pair_str.trim().split('-').collect();

//         if parts.len() != 2 {
//             fs::write("./highscore.txt", "0-0,0-0,0-0").unwrap();
//             return parse_highscore(&fs::read_to_string("./highscore.txt").unwrap());
//         }

//         let a = parts[0].trim().parse::<i64>().unwrap_or(0);
//         let b = parts[1].trim().parse::<i64>().unwrap_or(0);

//         result[i] = (a, b);
//     }

//     result
// }

fn main() {
    // nvm, i aint doin allat
    // if !Path::new("./highscore.txt").exists() {
    //     fs::write("./highscore.txt", "0-0,0-0,0-0").unwrap();
    // }

    start_game();
}
