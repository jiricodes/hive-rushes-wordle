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
        for i in 0..6 {
                let mut input = String::new();
                print!("Insert current guess:");
                let _ = stdout().flush();
                stdin().read_line(&mut input).expect("Incorrect input"); // TODO: Validate length and that word is not in discarded
                input.pop();
                input = input.to_lowercase();
                let mut status = String::new();
                print!("Insert status string [GYX]:");
                let _ = stdout().flush();
                stdin().read_line(&mut status).expect("Incorrect input"); // TODO: Validate length and that word is not in discarded
                status.pop();
                status = status.to_uppercase();
                assistant.update(&input, &status);
                assistant.display_suggestions();
        }
}
