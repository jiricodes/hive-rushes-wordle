use clap::{Arg as ClapArg, Command as ClapCommand};
use lib::assistant::Assistant;
use std::io::{stdin, stdout, Write};

/// Currently used for testing
fn main() {
	let args = ClapCommand::new("Wordle Assistant")
		.arg(ClapArg::new("dict").index(1))
		.after_help("Words dictionary")
		.get_matches();
	let path = args
		.value_of("dict")
		.expect("dict file expected as argument");
	let mut assistant = Assistant::new(path);
	for _ in 0..6 {
		let mut input = String::new();
		print!("\nInsert current guess:");
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
		if status == "GGGGG" {
			println!("Wordle solved.");
			break;
		}
		assistant.update(&input, &status);
		println!("");
		if assistant.suggestions_empty() {
			println!("Out of suggestions, did you win?");
			break;
		} else {
			assistant.display_suggestions();
			// println!("Best {}", assistant.get_most_freq().unwrap());
		}
	}
}
