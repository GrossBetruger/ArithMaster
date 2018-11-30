extern crate rand;

use rand::Rng;
use std::io;
use std::num::ParseIntError;

const NUM_OF_PRAISES: usize = 24;

const STREAK_CONSTANT: u8 = 5;

const UPDATE_DIFFICULTY: &str = "\nupdating difficulty to: {:?}\n";

const PRAISES: [&str; NUM_OF_PRAISES] = [
    "not bad!",
    "not too shabby...",
    "I'm starting to get impressed",
    "I'm impressed!",
    "Well done there!",
    "Wow!",
    "You're on fire!",
    "did you consider engineering?",
    "gotta love this accuracy!",
    "you're unstopable!!",
    "HAVE MERCY!",
    "is this even POSSIBLE?",
    "wtf! another hit?",
    "that's some math snipping",
    "you're possesed by deamons!",
    "I officially quit",
    "You make me look bad, and I'm a COMPUTER",
    "I'm your biggest fan",
    "MARRY ME!",
    "can you be more robust?!",
    "Is there a nobel prize for arithmetic?",
    "to say you're a bigshot would be such an understatement...",
    "Godlike skills !@!",
    "any futher praise would be an insult for you immanence",
];

enum Exercise {
    Addition,
}

#[derive(Debug)]
enum Difficulty {
    Easy,
    Medium,
    Hard
}

fn generate_random(min: i32, max: i32) -> i32 {
    rand::thread_rng().gen_range(min, max)
}

fn create_addition_exercise(min: i32, max: i32) -> (i32, i32, i32) {
    let a = generate_random(min, max);
    let b = generate_random(min, max);
    return (a, b, a + b);
}

fn interact_with_user(
    min: i32,
    max: i32,
    exercise: &Fn(i32, i32) -> (i32, i32, i32),
) -> Result<bool, ParseIntError> {
    let (operand_a, operand_b, solution) = exercise(min, max);
    println!("what is ({}) + ({})?", operand_a, operand_b);

    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("failed to read your answer");

    let input = answer.trim().parse::<i32>();
    match input {
        Ok(num) => {
            if num == solution {
                println!("very good!");
                return Ok(true);
            } else {
                println!("kinda wrong...");
                return Ok(false);
            }
        }
        Err(parse_int_err) => {
            println!("I don't speak that language...");
            Err(parse_int_err)
        }
    }
}

fn ask_question(exercise_type: Exercise, difficulty: &Difficulty) -> bool {
    match exercise_type {
        Exercise::Addition => match difficulty {
            Difficulty::Easy => {
                if let Ok(result) = interact_with_user(-10, 11, &create_addition_exercise) {
                    return result;
                } else {
                    return ask_question(exercise_type, difficulty);
                }
            }

            Difficulty::Medium => {
                if let Ok(result) = interact_with_user(-50, 51, &create_addition_exercise) {
                    return result;
                } else {
                    return ask_question(exercise_type, difficulty);
                }
            },
            Difficulty::Hard => {
                if let Ok(result) = interact_with_user(-400, 401, &create_addition_exercise) {
                    return result;
                } else {
                    return ask_question(exercise_type, difficulty);
                }
            }
        },
    }
    false
}

fn ask_forever() {
    let mut streak = 0;
    let mut praise_counter = 0;
    let mut current_difficulty = Difficulty::Easy;

    loop {

        match ask_question(Exercise::Addition, &current_difficulty) {
            true => {
                streak += 1;
                if streak % STREAK_CONSTANT == 0 {
                    let praise = PRAISES[praise_counter];
                    if praise_counter < NUM_OF_PRAISES - 1 {
                        praise_counter += 1;
                    }

                    println!("\n{} in a row! {}\n", streak, praise);
                }

                if streak % (STREAK_CONSTANT * 2) == 0 {
                    match current_difficulty {
                        Difficulty::Easy => {
                            let new_difficulty = Difficulty::Medium;
                            println!("\nupdating difficulty to: {:?}\n", new_difficulty);
                            current_difficulty = new_difficulty;
                        },
                        Difficulty::Medium => {
                            let new_difficulty = Difficulty::Hard;
                            println!("\nupdating difficulty to: {:?}\n", new_difficulty);
                            current_difficulty = new_difficulty;
                        },
                        _ => {}
                    }
                }
            }
            false => {
                streak = 0;
                praise_counter = 0;
                let new_difficulty = Difficulty::Easy;
                println!("\nupdating difficulty to: {:?}\n", new_difficulty);
                current_difficulty = new_difficulty;
            }
        }
    }
}

fn main() {
    ask_forever();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn random_generator() {
        for _ in 1..1000 {
            let random = generate_random(-3, 3);
            assert!(random < 3 && random > -4);
        }
    }
}
