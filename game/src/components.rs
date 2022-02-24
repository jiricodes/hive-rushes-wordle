use bevy::prelude::*;

#[derive(Component)]
pub struct Tile;

#[derive(Component, PartialEq, Clone, Copy, Debug)]
pub struct TilePosition {
    pub row: usize,
    pub col: usize,
}

#[derive(Component)]
pub struct TileSize {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Value(pub String);

#[derive(Component)]
pub struct TextTileValue;

#[derive(Component)]
pub struct VictoryScreen;

#[derive(Component)]
pub struct GameOverScreen;

#[derive(Component)]
pub struct IncorrectWordScreen;
