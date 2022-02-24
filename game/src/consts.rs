use bevy::prelude::Color;

// Tiles
pub const TILE_SIZE: f32 = 100.0;
pub const TILE_DEFAULT_COLOR: Color = Color::rgb(252.0 / 255.0, 255.0 / 255.0, 252.0 / 255.0);
pub const TILE_GREEN_COLOR: Color = Color::rgb(36.0 / 255.0, 130.0 / 255.0, 50.0 / 255.0);
pub const TILE_YELLOW_COLOR: Color = Color::rgb(255.0 / 255.0, 184.0 / 255.0, 0.0 / 255.0);
pub const TILE_GREY_COLOR: Color = Color::rgb(57.0 / 255.0, 61.0 / 255.0, 63.0 / 255.0);

// Font
pub const FONT_PATH: &str = "fonts/VCR_OSD_MONO.ttf";
pub const FONT_SIZE: f32 = 60.0;
pub const FONT_COLOR: Color = Color::rgb(0.0 / 255.0, 0.0 / 255.0, 0.0 / 255.0);

// Puzzle
pub const WIDTH: f32 = 5.0;
pub const HEIGHT: f32 = 6.0;
pub const BACKGROUND: Color = Color::rgba(0.15, 0.15, 0.15, 0.9);
