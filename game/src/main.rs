mod components;
mod consts;
mod game_plugin;
mod no_game_plugin;
mod resources;
mod states;
mod utils;

use game_plugin::GamePlugin;
use no_game_plugin::{GameOverPlugin, VictoryPlugin};
use resources::{CurrentGuess, Cursor, Game};
use states::GameState;

use bevy::prelude::*;
use clap::{Arg as ClapArg, Command as ClapCommand};

fn main() {
	let args = ClapCommand::new("add macro here")
		.arg(ClapArg::new("dict").index(1))
		.after_help("Words dictionary")
		.get_matches();
	let path = args
		.value_of("dict")
		.expect("dict file expected as argument");
	let game = Game::new(path);
	let cursor = Cursor::default();
	let guess = CurrentGuess::default();
	App::new()
		.insert_resource(Color::rgb(0.15, 0.15, 0.15))
		.insert_resource(WindowDescriptor {
			width: 800.0,
			height: 600.0,
			title: "Wordle".to_string(),
			..Default::default()
		})
		.insert_resource(game)
		.insert_resource(cursor)
		.insert_resource(guess)
		.add_plugins(DefaultPlugins)
		.add_state(GameState::InGame)
		.add_plugin(GamePlugin)
		.add_plugin(GameOverPlugin)
		.add_plugin(VictoryPlugin)
		.run();
}
