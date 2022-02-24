use crate::components::*;
use crate::consts::*;
use crate::states::GameState;
use crate::utils::despawn_screen;
use bevy::input::keyboard::{KeyCode, KeyboardInput};
use bevy::input::ElementState;
use bevy::prelude::*;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
	fn build(&self, app: &mut App) {
		app.add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(gameover_setup))
			.add_system_set(SystemSet::on_update(GameState::GameOver).with_system(gameover_update))
			.add_system_set(
				SystemSet::on_exit(GameState::GameOver)
					.with_system(despawn_screen::<GameOverScreen>)
					.with_system(gameover_exit),
			);
	}
}

fn gameover_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	let font_handle: Handle<Font> = asset_server.load(FONT_PATH);
	let text_style = TextStyle {
		font: font_handle,
		font_size: 100.0,
		color: Color::RED,
	};
	let text_alignment = TextAlignment {
		vertical: VerticalAlign::Center,
		horizontal: HorizontalAlign::Center,
	};
	commands
		.spawn_bundle(Text2dBundle {
			text: Text::with_section("GAME OVER".to_string(), text_style.clone(), text_alignment),
			transform: Transform {
				translation: Vec3::new(0.0, 0.0, 2.0),
				..Default::default()
			},
			..Default::default()
		})
		.insert(GameOverScreen);
}

fn gameover_update(mut keys: ResMut<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
	if keys.just_released(KeyCode::Escape) {
		state.set(GameState::Restarting).unwrap();
		keys.reset(KeyCode::Escape);
	}
}

fn gameover_exit(mut commands: Commands) {}

pub struct VictoryPlugin;

impl Plugin for VictoryPlugin {
	fn build(&self, app: &mut App) {
		app.add_system_set(SystemSet::on_enter(GameState::Victory).with_system(victory_setup))
			.add_system_set(SystemSet::on_update(GameState::Victory).with_system(victory_update))
			.add_system_set(
				SystemSet::on_exit(GameState::Victory)
					.with_system(despawn_screen::<VictoryScreen>)
					.with_system(victory_exit),
			);
	}
}

fn victory_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	let font_handle: Handle<Font> = asset_server.load(FONT_PATH);
	let text_style = TextStyle {
		font: font_handle,
		font_size: 100.0,
		color: Color::GREEN,
	};
	let text_alignment = TextAlignment {
		vertical: VerticalAlign::Center,
		horizontal: HorizontalAlign::Center,
	};
	commands
		.spawn_bundle(Text2dBundle {
			text: Text::with_section("VICTORY".to_string(), text_style.clone(), text_alignment),
			transform: Transform {
				translation: Vec3::new(0.0, 0.0, 2.0),
				..Default::default()
			},
			..Default::default()
		})
		.insert(VictoryScreen);
}

fn victory_update(mut keys: ResMut<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
	if keys.just_released(KeyCode::Escape) {
		state.set(GameState::Restarting).unwrap();
		keys.reset(KeyCode::Escape);
	}
}

fn victory_exit(mut commands: Commands) {}

pub struct IncorrectWordPlugin;

impl Plugin for IncorrectWordPlugin {
	fn build(&self, app: &mut App) {
		app.add_system_set(
			SystemSet::on_enter(GameState::IncorrectWord).with_system(incorrectword_setup),
		)
		.add_system_set(
			SystemSet::on_update(GameState::IncorrectWord).with_system(incorrectword_udate),
		)
		.add_system_set(
			SystemSet::on_exit(GameState::IncorrectWord)
				.with_system(despawn_screen::<IncorrectWordScreen>)
				.with_system(incorrectword_exit),
		);
	}
}

fn incorrectword_setup(mut commands: Commands) {}

fn incorrectword_udate(mut commands: Commands) {}

fn incorrectword_exit(mut commands: Commands) {}

fn text_size_system() {}
