use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
// Now loops. No longer calls itself over and over to cause a stack overflow.
// Instead, it just loops until the user decides to stop playing.
// I am learning.
    loop {
        play_game();

        println!("Would you like to play again? (yes/no)");

        let mut replay = String::new();
        io::stdin()
            .read_line(&mut replay)
            .expect("Failed to read line");
        // As long as the player doesn't say yes or y, the game will end.
        // If they do say yes, the loop resumes and starts again.
        // Stack overflow: PREVENTED!

        let replay = replay.trim().to_lowercase();
        if replay != "yes" && replay != "y" {
            println!("Thank you for playing! Goodbye!");
            stopPlaying();
        }
    }
}
fn play_game() {
    println!("Guess the number of pixies!");

    let pixies = rand::thread_rng().gen_range(1..=100);//immutable variable

    let mut guesses  = 5;
    let mut replay= String::new(); //Variable to help parse replay input
    while guesses > 0 {
        //If the user has lives left, prompt them to input a guess
        println!("Please input your guess.");
        let mut guess= String::new(); //mutable variable

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("You guessed: {guess}");

        //Uses match guess.cmp to determine if guess is greater/less/equal to pixies
        match guess.cmp(&pixies){
            Ordering::Less => {
                println!("Not enough pixies!");
                guesses = guesses -1;
                println!("You have {} guesses left.", guesses);
            }
            Ordering::Greater => {
                println!("Too many pixies!");
                guesses = guesses -1;
                println!("You have {} guesses left.", guesses);
            }
            Ordering::Equal => {
                println!("You win! Keep on sparkling, darling! Would you like to play again?");
                io::stdin()
                .read_line(&mut replay)
                .expect("Failed to read line");
            }
        }

    }
        
            //If the user has no lives left, end the game.
            println!("You have run out of guesses! The redcaps have come for you! Would you like to play again?");

        }
    

fn stopPlaying() {
    std::process::exit(0); //Exit the program gracefully
}
