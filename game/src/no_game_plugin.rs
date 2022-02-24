use crate::states::GameState;
use bevy::prelude::*;

pub struct GameOver;

impl Plugin for GameOver {
	fn build(&self, app: &mut App) {}
}

fn gameover_setup(mut commands: Commands) {}

fn gameover_exit(mut commands: Commands) {}

pub struct Victory;

impl Plugin for Victory {
	fn build(&self, app: &mut App) {}
}

fn victory_setup(mut commands: Commands) {}

fn victory_exit(mut commands: Commands) {}

pub struct IncorrectWord;

impl Plugin for IncorrectWord {
	fn build(&self, app: &mut App) {}
}

fn incorrectword_setup(mut commands: Commands) {}

fn incorrectword_exit(mut commands: Commands) {}

fn text_size_system() {}
