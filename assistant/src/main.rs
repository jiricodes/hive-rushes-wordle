mod assistant;
mod suggestion;

use assistant::Assistant;
use clap::{Arg as ClapArg, Command as ClapCommand};
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
    let mut input = String::new();
    print!("Insert current guess:");
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Incorrect input"); // TODO: Validate length and that word is not in discarded
    println!("Input: {}", input);
    assistant.update(&input);
    assistant.display_suggestions();
}
