use clap::{Arg as ClapArg, Command as ClapCommand};
use lib::assistant::Assistant;
use std::io::{stdin, stdout, Write};

/// Currently used for testing
fn main() {
    let args = ClapCommand::new("add macro here")
        .arg(ClapArg::new("dict").index(1))
        .after_help("Words dictionary")
        .get_matches();
    let path = args
        .value_of("dict")
        .expect("dict file expected as argument");
    let mut assistant = Assistant::new(path);
    for _ in 0..6 {
        let mut word = String::new();
        let guess = assistant.get_random();
        if guess.is_some() {
            word = guess.unwrap();
        } else {
            println!("Out of suggestions, did you win?");
            break;
        }
        println!("Try this next: {}", word);
        let mut status = String::new();
        print!("Insert status string [GYX]:");
        let _ = stdout().flush();
        stdin().read_line(&mut status).expect("Incorrect input"); // TODO: Validate length and that word is not in discarded
        status.pop();
        status = status.to_uppercase();
        assistant.update(&word, &status);
    }
}
