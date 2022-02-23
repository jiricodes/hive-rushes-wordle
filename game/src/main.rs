mod components;
mod game_plugin;
mod gameover_plugin;
mod states;
mod utils;
use game_plugin::GamePlugin;
use states::GameState;

use bevy::prelude::*;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_state(GameState::InGame)
		.add_plugin(GamePlugin)
		.run();
}
