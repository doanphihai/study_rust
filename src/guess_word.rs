use std::ops::ControlFlow;
use rand::Rng;

pub(crate) fn guess_word() {
    let sceret_number: u32 = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut buf = String::new();

    loop {
        // read user input
        io::stdin().read_line(&mut buf).expect("Failed to read line");
        println!("You guessed: {}", buf);
        // parse the guess
        let trim = &buf.trim();
        // check if the user input is empty
        if trim.is_empty() {
            println!("You didn't guess anything!");
            buf.clear();
            // skip the rest of the loop
            break;
        }
        let guess: u32 = trim.parse().expect("Please type a number!");
        if let ControlFlow::Break(_) = guess_logic_2(guess, sceret_number) {
            break;
        }
        // clear the buffer
        buf.clear();
    }
    
    println!("sceret number: {}", sceret_number);
}

pub(crate) fn guess_logic_1(guess: u32, sceret_number: u32) -> ControlFlow<()> {
    // check if the user input is a number
    if guess == sceret_number {
        println!("You guessed right!");
        // skip the rest of the loop
        return ControlFlow::Break(());
    } else if guess > sceret_number {
        println!("You guessed too big!");
    } else if guess < sceret_number {
        println!("You guessed too small!");
    }
    ControlFlow::Continue(())
}

pub(crate) fn guess_logic_2(guess: u32, sceret_number: u32) -> ControlFlow<()> {
    // check if the user input is a number
    match guess.cmp(&sceret_number) {
        // check if the guess is equal to the sceret number
        std::cmp::Ordering::Equal => {
            println!("You guessed right!");
            // skip the rest of the loop
            return ControlFlow::Break(());
        },
        // check if the guess is greater than the sceret number
        std::cmp::Ordering::Greater => {
            println!("You guessed too big!");
        },
        // check if the guess is less than the sceret number
        std::cmp::Ordering::Less => {
            println!("You guessed too small!"); 
        }
    }
    // continue the loop
    ControlFlow::Continue(())
}
