use clap::{Arg, Command};
use lib::database::Database;

/// Currenly used for testing
fn main() {
    let args = Command::new("add macro here")
        .arg(Arg::new("dict").index(1))
        .after_help("Words dictionary")
        .get_matches();
    let path = args
        .value_of("dict")
        .expect("dict file expected as argument");
    let mut d = Database::load(path);
    d.prune_yellow('w', 0);
    d.prune_yellow('i', 1);
    d.prune_yellow('s', 2);
    d.prune_yellow('e', 3);
    d.prune_yellow('r', 4);

    dbg!(d.get_available());
}
