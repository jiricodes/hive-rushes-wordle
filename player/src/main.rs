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
    d.prune_grey('h');
    d.prune_grey('l');
    d.prune_yellow('e', 1);
    d.prune_yellow('o', 4);
    d.prune_green('v', 3);
    d.prune_green('e', 4);
    d.prune_grey('g');
    d.prune_green('o', 2);
    d.prune_grey('a');
    d.prune_grey('b');
    d.prune_grey('d');
    dbg!(d.get_available());
}
