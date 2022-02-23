use crate::states::GameState;
use bevy::prelude::*;

pub struct GameOver;

impl Plugin for GameOver {
	fn build(&self, app: &mut App) {}
}

fn gameover_setup(mut commands: Commands) {}

pub struct Victory;

impl Plugin for Victory {
	fn build(&self, app: &mut App) {}
}
