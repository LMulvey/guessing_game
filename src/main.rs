use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut prompt = "Please input your guess.";

    loop {
        println!("{}", prompt);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, 
            Err(_) => {
                prompt = "Please enter a number and try again!";
                continue;
            }
        };

        println!("You guessed: {}", guess);

        // resassign prompt message
        prompt = "Please input your guess.";

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!!!"),
            Ordering::Equal => {
              println!("You win, great job!");
              break;
            }
        }
    }
}
