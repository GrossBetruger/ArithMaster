extern crate rand;

use rand::Rng;
use std::io;

enum Exercise {
    Addition
}

enum Difficulty {
    Easy
}


fn generate_random(min: i32, max: i32) -> i32 {
    rand::thread_rng().gen_range(min, max)
}

fn create_addition_exercise(min: i32, max: i32) -> (i32, i32, i32){
    let a = generate_random(min, max);
    let b = generate_random(min, max);
    return (a, b, a + b)
}

fn ask_question(exercise_type: Exercise, difficulty: Difficulty) -> bool {
    match exercise_type {
        Exercise::Addition => {
            match difficulty {
                Difficulty::Easy => {
                    let (operand_a, operand_b, solution) =
                        create_addition_exercise(-10, 10);
                    println!("what is {} + {}?", operand_a, operand_b);

                    let mut answer = String::new();
                        io::stdin().read_line(&mut answer)
                            .expect("failed to read your answer");

                    let input = answer.trim().parse::<i32>();
                    if let Ok(num) = input {
                        if num == solution {
                            println!("very good!");
                            return true;
                        }
                        else { println!("kinda wrong...")}

                    }
                    else { return ask_question(exercise_type, difficulty); }

                }
            }

        },

    }
    false
}

fn ask_forever() {
    let mut streak = 0;
    loop {
        match ask_question(Exercise::Addition, Difficulty::Easy) {
            true => {
                    streak += 1;
                    if streak % 5 == 0 {
                        let praise = "not bad!";
                        println!("{} in a row! {}", streak, praise);
                    }
                }
            false => {streak = 0}
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