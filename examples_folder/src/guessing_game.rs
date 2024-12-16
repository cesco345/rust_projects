use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    let mut how_many = String::new();
    println!("How many random numbers do you want to guess?");
    io::stdin()
        .read_line(&mut how_many)
        .expect("Error reading input");

    let num_guesses: u8 = how_many.trim().parse().expect("Error reading input");

    let mut correct = Vec::new();

    for _ in 0..num_guesses {
        correct.push(rand::thread_rng().gen_range(1..=10));
    }

    //println!("{correct:?}");

    let mut guesses_made = 0;

    println!("Hey, guess a number 1-10:");
    while guesses_made < num_guesses {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error with parse, try again. {e}");
                continue;
            }
        };

        match guess.cmp(&correct[guesses_made as usize]) {
            Ordering::Greater => println!("You guessed too high."),
            Ordering::Less => println!("You guessed too low"),
            Ordering::Equal => {
                println!("You guessed CORRECT!");
                guesses_made += 1;
                if guesses_made < num_guesses {
                    println!("Let's now try the next number!");
                }
            }
        };
    }
    println!("Thanks for playing! The correct answers were:");
    for item in correct {
        println!("{item}");
    }
}
