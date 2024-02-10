use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let rand_num = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("you guessed: {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //comparing
        match guess.cmp(&rand_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You are correct!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        }
    }
}
