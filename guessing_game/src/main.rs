use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main () {
  println!("Lets play guess the number!");
  println!("The game will get harder with each round, so don't leave if the first round's too easy!");

  'game: for i in 1..10 {
    let max = 10u32.pow(i);
    let secret_number = rand::thread_rng().gen_range(1, max + 1);

    println!("I'm thinking of a number between 1 and {}", max);
    'round: loop {
      println!("Please input your guess.");

      let mut guess = String::new();

      io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

      guess = String::from(guess.trim());

      let guess: u32 = match guess.parse() {
        Ok(num) => num,
        Err(_) => {
          println!("{} is not a number.", guess);
          continue;
        },
      };

      match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too large"),
        Ordering::Equal => {
          println!("You win!");
          loop {
            println!("Do you want to play again? (y/n)");
            let mut new_game = String::new();
            io::stdin()
              .read_line(&mut new_game)
              .expect("Failed to read line");
            match new_game.trim() {
              "y" => break 'round,
              "n" => break 'game,
              _ => println!("Invalid input. Please enter \"y\" or \"n\""),
            }
          }
        }
      }
    }
  }

  println!("Thanks for playing!");
}