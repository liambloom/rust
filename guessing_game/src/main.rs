use std::io; // import the standard (std) module io  
use rand::Rng;
use std::cmp::Ordering;

fn main () {
  println!("Lets play guess the number!"); // calls the macro (from exclamation point) println
  println!("The game will get harder with each round, so don't leave if the first round's too easy!");

  'game: for i in 1..10 { // labels start with an apostrophe, for i in 1..10 is equivalent to for (let i = 0; i < 10; i++)
    let max = 10u32.pow(i); // 10u32 = unsigned 32 bit int 10
    let secret_number = rand::thread_rng().gen_range(1, max + 1); /*
    thread_rng is defined in the Rng trait of rand (idk what that means)
    static property treat_rng of rand, which is a rng generator on the thread this is running on
    gen_range is a property that generates a random number between arg 1 (inclusive) and arg 2 (exclusive)*/

    println!("I'm thinking of a number between 1 and {}", max);
    'round: loop { // creates an infinite loop
      println!("Please input your guess.");

      let mut guess = String::new(); /*
      let declares a variable
      variables are constant by default, `mut` makes them mutable
      String is a type
      :: calls a *static* method of String
      ::new() calls the static method `new` of type `String`
      String::new() creates a new string ""
      It is standard for new to be the name of the constructor, but not required
      Rust is strongly, statically typed, but it can infer types, so this is a String*/

      io::stdin() /*
      Returns a new std::io::Stdin. 
      Instead of importing it at the start, I could have used std::io::stdin() */
        .read_line(&mut guess) /*
        & says that this is a reference
        &mut declares a mutable reference (references are NOT the variable, and are also immutable by default)
        an immutable reference to guess could be created using &guess
        read_line is a method that puts the value from the input into the variable guess 
        read_line returns type std::io::Result, which is a type of enum*/
        .expect("Failed to read line");
        // the expect method, if there is an error, crashes the program and displays the message passed into it (why is this not a feature in js!?!?!)

      let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
          println!("{} is not a number.", guess.trim());
          continue;
        },
      }; /*
      this creates a new variable guess of type u32 (unsigned 32 bit int)
      this is a new guess variable, it "shadows" the previous variable, meaning that it is a new variable with the same name
      the trim method removes the newline at the end of the input
      the parse input converts it to a number*/

      match guess.cmp(&secret_number) { /*
        guess.cmp is a method that compares guess to an argument, in this case, a reference too secret_number
        It then returns a variant of the Ordering enum, either Less, Greater, or Equal
        the match expression is made up of "arms"
        it checks each arm and compares it to the entered value (returned from guess.cmp(&secret_number), kind of like a switch statement
        when an arm is matched, the code runs and the match statement ends*/
        Ordering::Less => println!("Too small"), // std::cmp::Ordering
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