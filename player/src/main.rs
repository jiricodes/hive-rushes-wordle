use clap::{Arg, Command};
use lib::database::Database;

fn main() {
    let args = Command::new("add macro here")
        .arg(Arg::new("dict").index(1))
        .after_help("Words dictionary")
        .get_matches();
    let path = args
        .value_of("dict")
        .expect("dict file expected as argument");
    let mut d = Database::load(path);
    dbg!(&d);
    let w = d.get_random();
    println!("Random word: {}", w);
    dbg!(&d);
    d.prune_letter_at_position('c', 3);
    dbg!(&d);
}
