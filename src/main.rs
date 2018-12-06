extern crate termcolor;
extern crate colored;
extern crate rand;

use colored::*;
use rand::Rng;
use std::io;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::BufRead;
use std::io::BufReader;
use std::num::ParseFloatError;


const NUM_OF_PRAISES: usize = 24;

const SUPER_PRAISES_THRESHOLD: u8 = 45;

const STREAK_CONSTANT: u8 = 5;

const EPSILON: f32 = 0.01;

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
    "any futher praise would be an insult to your immanence",
];

enum Exercise {
    Addition,
    Subtraction,
    Multiplication,
    Exponentiation,
    Division,
}

#[derive(Debug)]
enum Difficulty {
    Easy,
    Medium,
    Hard,
}

fn generate_random(min: i32, max: i32) -> i32 {
    rand::thread_rng().gen_range(min, max)
}

fn create_addition_exercise(min: i32, max: i32) -> (i32, i32, f32) {
    let a = generate_random(min, max);
    let b = generate_random(min, max);
    return (a, b, (a + b) as f32);
}

fn create_subtraction_exercise(min: i32, max: i32) -> (i32, i32, f32) {
    let a = generate_random(min, max);
    let b = generate_random(min, max);
    return (a, b, (a - b) as f32);
}

fn create_multiplication_exercise(min: i32, max: i32) -> (i32, i32, f32) {
    let a = generate_random(min as i32, max as i32);
    let b = generate_random(min as i32, max as i32);
    return (a, b, (a * b) as f32);
}

fn create_exponentiation_exercise(min: i32, max: i32) -> (i32, i32, f32) {
    let a = generate_random(0, 11);
    let e = generate_random(min, max).abs();
    return (a, e, a.pow(e as u32) as f32);
}

fn create_division_exercise(min: i32, max: i32) -> (i32, i32, f32) {
    let a = generate_random(min as i32, max as i32);
    let b = generate_random(min as i32, max as i32);
    if b == 0 {
        return create_division_exercise(min, max);
    }
    return (a, b, a as f32 / b as f32);
}

fn format_superscript(base: i32, exponent: i32) -> String {
    match exponent {
        0 => {return format!("{}{}", base, "\u{02070}")},
        1 => {return format!("{}{}", base, "\u{00b9}")},
        2 => {return format!("{}{}", base, "\u{00b2}")},
        3 => {return format!("{}{}", base, "\u{00b3}")},
        4 => {return format!("{}{}", base, "\u{02074}")},
        5 => {return format!("{}{}", base, "\u{02075}")},
        6 => {return format!("{}{}", base, "\u{02076}")},
        7 => {return format!("{}{}", base, "\u{02077}")},
        8 => {return format!("{}{}", base, "\u{02078}")},
        9 => {return format!("{}{}", base, "\u{02079}")},
        _ => {}

    }
    "nothing".into()
}

fn print_spaced(printable: &str) {
    println!("\n{}\n", printable)
}

fn warn(printable: &str) -> String {
    format!("{}", printable.yellow())
}

fn shout(printable: &str) -> String {
    format!("{}", printable.red())
}

fn say_minor_praise(printable: &str) -> String {
    format!("{}", printable.green())
}

fn say_praise(printable: &str) -> String {
    format!("{}", printable.bright_magenta())
}

fn say_super_praise(printable: &str) -> String {
    format!("{}", printable.purple())
}

fn bracket_negative(num: i32) -> String {
    match num < 0 {
        true => format!("({})", num),
        false => format!("{}", num)
    }
}

fn format_question(operation: &str, operand_a: i32, operand_b: i32) -> String {
    match operation {
        "^" => {format!("what is {}?", format_superscript(operand_a, operand_b))}
        _ => {
            format!("what is {} {} {}?", bracket_negative(operand_a),
                    operation, bracket_negative(operand_b))
        }
    }
}

fn read_user_answer<R: BufRead>(mut reader: R) -> Result<f32, ParseFloatError> {
//    let mut answer = String::new();
    let mut answer = vec![];
    reader
        .read_until(0xa, &mut answer)
        .expect("failed to read your answer");
    String::from_utf8_lossy(&answer).trim().parse::<f32>()
}

fn check_answer(user_num: f32, solution: f32) -> bool {
    (user_num - solution).abs() < EPSILON
}

fn interact_with_user(
    operation: &str,
    min: i32,
    max: i32,
    exercise: &Fn(i32, i32) -> (i32, i32, f32),
) -> Result<bool, ParseFloatError> {
    let (operand_a, operand_b, solution) = exercise(min, max);
    println!("{}", format_question(operation, operand_a, operand_b));

    let reader = BufReader::new(io::stdin());
    let input = read_user_answer(reader);
    match input {
        Ok(num) => {
            if check_answer(num, solution) {
                print_spaced(&say_minor_praise("very good!"));
                return Ok(true);
            } else {
                print_spaced(&format!(
                    "{} it's more like: {}",
                    shout("kinda wrong..."),
                    solution.to_string().yellow()
                ));
                return Ok(false);
            }
        }
        Err(parse_float_err) => {
            print_spaced(&warn("I don't speak that language"));
            Err(parse_float_err)
        }
    }
}

fn ask_question(exercise_type: &Exercise, difficulty: &Difficulty) -> bool {
    let (add_sub_min, add_sub_max) = match difficulty {
        Difficulty::Easy => (-10, 11),
        Difficulty::Medium => (-50, 51),
        Difficulty::Hard => (-400, 401),
    };

    let (mul_div_min, mul_div_max) = match difficulty {
        Difficulty::Easy => (-10, 11),
        Difficulty::Medium => (-25, 26),
        Difficulty::Hard => (-125, 126),
    };

    let (exp_min, exp_max) = match difficulty {
        Difficulty::Easy => (0, 3),
        Difficulty::Medium => (3, 6),
        Difficulty::Hard => (6, 8),
    };

    match exercise_type {
        Exercise::Addition => {
            if let Ok(result) =
                interact_with_user("+", add_sub_min, add_sub_max, &create_addition_exercise)
            {
                return result;
            } else {
                return false //ask_question(exercise_type, difficulty);
            }
        }
        Exercise::Subtraction => {
            if let Ok(result) =
                interact_with_user("-", add_sub_min, add_sub_max, &create_subtraction_exercise)
            {
                return result;
            } else {
                return false //ask_question(exercise_type, difficulty);
            }
        }
        Exercise::Multiplication => {
            if let Ok(result) = interact_with_user(
                "*",
                mul_div_min,
                mul_div_max,
                &create_multiplication_exercise,
            ) {
                return result;
            } else {
                return false // ask_question(exercise_type, difficulty);
            }
        }
        Exercise::Division=> {
            if let Ok(result) = interact_with_user(
                "/",
                mul_div_min,
                mul_div_max,
                &create_division_exercise,
            ) {
                return result;
            } else {
                return false // ask_question(exercise_type, difficulty);
            }
        }
        Exercise::Exponentiation=> {
            if let Ok(result) = interact_with_user(
                "^",
                exp_min,
                exp_max,
                &create_exponentiation_exercise,
            ) {
                return result;
            } else {
                return false//ask_question(exercise_type, difficulty);
            }
        }
    }
}

fn ask_forever() {
    let mut streak = 0;
    let mut praise_counter = 0;
    let mut current_difficulty = Difficulty::Easy;
    let mut current_question_type = Exercise::Addition;

    loop {
        match ask_question(&current_question_type, &current_difficulty) {
            true => {
                streak += 1;
                if streak % STREAK_CONSTANT == 0 {
                    let praise = PRAISES[praise_counter];
                    if praise_counter < NUM_OF_PRAISES - 1 {
                        praise_counter += 1;
                    }

                    let praise = format!("\n{} in a row! {}\n", streak, praise);
                    match streak < SUPER_PRAISES_THRESHOLD {
                        true => print_spaced(&say_praise(&praise)),
                        _ => print_spaced(&say_super_praise(&praise)),
                    }

                    format!("\n{} in a row! {}\n", streak, praise);
                }

                if streak % (STREAK_CONSTANT * 2) == 0 {
                    match current_difficulty {
                        Difficulty::Easy => {
                            let new_difficulty = Difficulty::Medium;
                            print_spaced(&say_minor_praise(&format!(
                                "updating difficulty to: {:?}",
                                new_difficulty
                            )));
                            current_difficulty = new_difficulty;
                        }
                        Difficulty::Medium => {
                            let new_difficulty = Difficulty::Hard;
                            print_spaced(&say_minor_praise(&format!(
                                "updating difficulty to: {:?}",
                                new_difficulty
                            )));
                            current_difficulty = new_difficulty;
                        }
                        _ => {}
                    }
                }
            }
            false => {
                streak = 0;
                praise_counter = 0;
                let new_difficulty = Difficulty::Easy;
                print_spaced(&warn(&format!(
                    "updating difficulty to: {:?}",
                    new_difficulty
                )));
                current_difficulty = new_difficulty;
            }
        }
        //        change exercise type on random
        match generate_random(0, 5) {
            0 => current_question_type = Exercise::Addition,
            1 => current_question_type = Exercise::Multiplication,
            2 => current_question_type = Exercise::Exponentiation,
            3 => current_question_type = Exercise::Division,
            _ => current_question_type = Exercise::Subtraction,
        }
    }
}

fn main() {
//    set stdout to colored (for windows systems)
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
    writeln!(&mut stdout, "hello there!\n").unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White))).unwrap();

    ask_forever();
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs::File;
    use std::fs::OpenOptions;
    use std::fs::remove_file;

    #[test]
    fn random_generator() {
        for _ in 0..1000 {
            let random = generate_random(-3, 3);
            assert!(random < 3 && random > -4);
        }
    }

    #[test]
    fn addition() {
        for _ in 0..1000 {
            let (a, b, res) = create_addition_exercise(-1000, 1001);
            assert_eq!((a + b) as f32, res);
        }
    }

    #[test]
    fn subtraction() {
        for _ in 0..1000 {
            let (a, b, res) = create_subtraction_exercise(-1000, 1001);
            assert_eq!((a - b) as f32, res);
        }
    }

    #[test]
    fn multiplication() {
        for _ in 0..1000 {
            let (a, b, res) = create_multiplication_exercise(-1000, 1001);
            assert_eq!((a * b) as f32, res);
        }
    }

    #[test]
    fn exponentiation() {
        for _ in 0..1000 {
            let (a, e, res) = create_exponentiation_exercise(0, 10);
            assert_eq!((a.pow(e as u32)) as f32, res);
        }
    }

    #[test]
    fn superscript() {
        assert_eq!("10⁰", format_superscript(10, 0));
        assert_eq!("10¹", format_superscript(10, 1));
        assert_eq!("10²", format_superscript(10, 2));
        assert_eq!("10³", format_superscript(10, 3));
        assert_eq!("10⁴", format_superscript(10, 4));
        assert_eq!("10⁵", format_superscript(10, 5));
        assert_eq!("10⁶", format_superscript(10, 6));
        assert_eq!("10⁷", format_superscript(10, 7));
        assert_eq!("10⁸", format_superscript(10, 8));
        assert_eq!("10⁹", format_superscript(10, 9));
    }

    fn test_exercise(mock_stdin_path: &str, exercise: &Fn(i32, i32) -> (i32, i32, f32), min: i32, max: i32) {
        let mut file = File::create(mock_stdin_path).expect("error creating mock file");
        let (_, _, res) = exercise(min, max);
        let str_repr = res.to_string();
        let input: &[u8] = str_repr.as_bytes();
        file.write(input).expect("filed to write number to file");

        match OpenOptions::new().create(false).read(true).open(mock_stdin_path) {
            Ok(ref mut file) => {
                 match read_user_answer(BufReader::new(file)) {
                     Ok(answer) => assert_eq!(answer, res),
                     _ => panic!("failed to read mock answer")
                 }
            },
            Err(err) => { panic!("Failed to open mock file: {}", err); }
        }

        remove_file(mock_stdin_path).expect("failed to remove mock file");
    }

    #[test]
    fn user_interaction_addition() {
        for _ in 1..1000 {
            let path = "mock.addition.stdin";
            test_exercise(path, &create_addition_exercise, -1000, 1000);
        }

    }

    #[test]
    fn user_interaction_subtraction() {
        for _ in 1..1000 {
            let path = "mock.subtraction.stdin";
            test_exercise(path, &create_subtraction_exercise, -1000, 1000);
        }

    }

    #[test]
    fn user_interaction_multiplication() {
        for _ in 1..1000 {
            let path = "mock.multiplication.stdin";
            test_exercise(path, &create_multiplication_exercise, -1000, 1000);
        }

    }

    #[test]
    fn user_interaction_exponentiation() {
        for _ in 1..1000 {
            let path = "mock.exponentiation.stdin";
            test_exercise(path, &create_exponentiation_exercise, 0, 10);
        }
    }

    #[test]
    fn user_interaction_division() {
        for _ in 1..1000 {
            let path = "mock.division.stdin";
            test_exercise(path, &create_division_exercise, -100, 100);
        }
    }

    #[test]
    fn question_formatting_addition() {
        assert_eq!("what is 6 + (-10)?", format_question("+", 6, -10));
    }

    #[test]
    fn question_formatting_subtraction() {
        assert_eq!("what is 3 - (-4)?", format_question("-", 3, -4));
    }

    #[test]
    fn question_formatting_subtraction_both_negatives() {
        assert_eq!("what is (-1) - (-3)?", format_question("-", -1, -3));
    }

    #[test]
    fn question_formatting_multiplication_no_negatives() {
        assert_eq!("what is 3 * 2?", format_question("*", 3, 2));
    }

    #[test]
    fn question_formatting_multiplication() {
        assert_eq!("what is (-8) * 4?", format_question("*", -8, 4));
    }

    #[test]
    fn question_formatting_exponentiation() {
        assert_eq!("what is 1²?", format_question("^", 1, 2));
    }

    #[test]
    fn answer_f34_precision() {
        assert!(check_answer(0.66, 2./3.), "0.66 did not equal to 2/3");
        assert!(check_answer(-0.33, -1./3.), "-0.33 did not equal to -1/3");
        assert!(check_answer(-0.66, 2./-3.), "-0.66 did not equal to 2/-3");
        assert!(! check_answer(0.65, 2./3.), "0.65 equaled 2/3");
        assert!(check_answer(1.25, 10./8.), "1.25 did not equal 10/8");
    }
}
