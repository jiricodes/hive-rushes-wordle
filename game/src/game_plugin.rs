use crate::components::*;
use bevy::input::keyboard::{KeyCode, KeyboardInput};
use bevy::input::ElementState;
use bevy::prelude::*;
use clap::{Arg as ClapArg, Command as ClapCommand};
use lib::database::Database;
use lib::game::{status_green, LetterStatus, WordStatus, Wordle};
use std::fmt;
use std::fmt::Debug;
use std::path::Path;

/// Enum to express gamestatus / state
pub enum GameStatus<T> {
    Ok(T),
    InvalidWord,
    GameOver,
    Victory(T),
}

/// Core Wordle Game struct
///
/// wordle provides the rules api and guess feedback
/// database can be used for word suggestions etc.
pub struct Game {
    wordle: Wordle,
    database: Database,
    pub guesses: Vec<Option<String>>,
    pub colors: Vec<Vec<Color>>,
}

impl Game {
    /// Constructor that requires path to database
    ///
    /// TODO: change this perhaps to an object with `database` trait or similar
    pub fn new<P>(filename: P) -> Self
    where
        P: AsRef<Path> + Debug,
    {
        let database = Database::load(filename);
        let word = database.get_random();
        println!("Wordle game with: {}", word);
        let wordle = Wordle::new(word);
        let limit = wordle.get_max_attempts();
        Self {
            wordle,
            database,
            guesses: vec![None; limit],
            colors: vec![vec![TILE_DEFAULT_COLOR; 5]; limit],
        }
    }

    pub fn make_guess_simple(&mut self, word: &String) -> GameStatus<Vec<Color>> {
        if !self.database.contains(word) {
            return GameStatus::InvalidWord;
        }
        if self.wordle.game_over() {
            return GameStatus::GameOver;
        }
        let status = &self.wordle.guess_word(word);
        if status_green(&status) {
            return GameStatus::Victory(status_as_colors(&status));
        } else {
            GameStatus::Ok(status_as_colors(&status))
        }
    }
}

fn status_as_colors(status: &WordStatus) -> Vec<Color> {
    let mut colors: Vec<Color> = Vec::new();
    for ls in status.iter() {
        let color = match ls {
            LetterStatus::Grey => TILE_GREY_COLOR,
            LetterStatus::Yellow => TILE_YELLOW_COLOR,
            LetterStatus::Green => TILE_GREEN_COLOR,
        };
        colors.push(color);
    }
    colors
}

struct Cursor {
    position: TilePosition,
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            position: TilePosition { row: 0, col: 0 },
        }
    }
}

#[derive(Default)]
struct CurrentGuess {
    pub word: String,
}

// Tiles
const TILE_SIZE: f32 = 100.0;
const TILE_DEFAULT_COLOR: Color = Color::rgb(252.0 / 255.0, 255.0 / 255.0, 252.0 / 255.0);
const TILE_GREEN_COLOR: Color = Color::rgb(36.0 / 255.0, 130.0 / 255.0, 50.0 / 255.0);
const TILE_YELLOW_COLOR: Color = Color::rgb(255.0 / 255.0, 184.0 / 255.0, 0.0 / 255.0);
const TILE_GREY_COLOR: Color = Color::rgb(57.0 / 255.0, 61.0 / 255.0, 63.0 / 255.0);

// Font
const FONT_PATH: &str = "fonts/VCR_OSD_MONO.ttf";
const FONT_SIZE: f32 = 60.0;
const FONT_COLOR: Color = Color::rgb(0.0 / 255.0, 0.0 / 255.0, 0.0 / 255.0);

// Puzzle
const WIDTH: f32 = 5.0;
const HEIGHT: f32 = 6.0;
const BACKGROUND: Color = Color::rgba(0.15, 0.15, 0.15, 0.9);
const MARGIN: f32 = TILE_SIZE * 0.05;
// const WIN_SIZE: f32 = MARGIN * (SIZE + 1.0) + TILE_SIZE * SIZE;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
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
        app.insert_resource(WindowDescriptor {
            width: 800.0,
            height: 600.0,
            title: "Wordle".to_string(),
            ..Default::default()
        })
        .insert_resource(game)
        .insert_resource(cursor)
        .insert_resource(guess)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(tile_size_system)
                .with_system(tile_position_system)
                .with_system(tile_color_system),
        )
        .add_startup_system(camera_setup)
        .add_startup_system(setup)
        .add_system(keyboard_input);
    }
}

/// System to handle resizing tiles based on window size
fn tile_size_system(win: Res<Windows>, mut q: Query<(&TileSize, &mut Sprite), With<Tile>>) {
    let w = win.get_primary().unwrap().width() as f32;
    let h = win.get_primary().unwrap().height() as f32;
    for (size, mut sprite) in q.iter_mut() {
        let sx = (w / WIDTH) * size.x;
        let sy = (h / HEIGHT) * size.y;
        sprite.custom_size = Some(Vec2::new(sx, sy));
    }
}

/// System to handle tiles positioning based - translates grid based to window based locations
fn tile_position_system(
    win: Res<Windows>,
    mut q: Query<(&mut Transform, &TilePosition), With<Tile>>,
) {
    let w = win.get_primary().unwrap().width() as f32;
    let h = win.get_primary().unwrap().height() as f32;
    for (mut tx, pos) in q.iter_mut() {
        let x = (pos.col as f32 / WIDTH) * w - w / 2.0 + (w / WIDTH) / 2.0;
        let y = -1.0 * (pos.row as f32 / HEIGHT) * h + h / 2.0 - (h / HEIGHT) / 2.0;
        tx.translation = Vec3::new(x, y, 0.0);
    }
}

fn tile_color_system(game: Res<Game>, mut q: Query<(&mut Sprite, &TilePosition), With<Tile>>) {
    for (mut sprite, pos) in q.iter_mut() {
        sprite.color = game.colors[pos.row][pos.col];
    }
}

/// Camera setup
fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

/// Game setup handler
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, game: Res<Game>) {
    let font_handle: Handle<Font> = asset_server.load(FONT_PATH);
    let text_style = TextStyle {
        font: font_handle,
        font_size: FONT_SIZE,
        color: FONT_COLOR,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    for row in 0..HEIGHT as usize {
        let guessed = game.guesses[row].as_ref();
        for col in 0..WIDTH as usize {
            let value = match guessed {
                Some(val) => val.chars().nth(col).unwrap(),
                None => ' ',
            };
            let label = format!("{}", value);
            // println!("[{}, {}]: {}", row, col, label);
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: TILE_DEFAULT_COLOR,
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(Text2dBundle {
                            text: Text::with_section(
                                label.clone(),
                                text_style.clone(),
                                text_alignment,
                            ),
                            transform: Transform {
                                translation: Vec3::new(0.0, 0.0, 1.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(TextTileValue)
                        .insert(TilePosition { row, col });
                })
                .insert(Tile)
                .insert(Value(label))
                .insert(TileSize { x: 0.95, y: 0.95 })
                .insert(TilePosition { row, col });
        }
    }
}

/// This currently contains all the logic, which shouldn't be the case
fn keyboard_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut guess: ResMut<CurrentGuess>,
    mut cursor: ResMut<Cursor>,
    mut value_q: Query<(&mut Value, &TilePosition), With<Tile>>,
    mut text_q: Query<(&mut Text, &TilePosition), With<TextTileValue>>,
    mut game: ResMut<Game>,
) {
    for ev in char_evr.iter() {
        // println!("Got char: '{}'", ev.char);
        if ev.char.is_ascii_alphabetic() && guess.word.len() < 5 {
            let label = format!("{}", ev.char).to_uppercase();
            guess.word.push(ev.char);
            for (mut text, pos) in text_q.iter_mut() {
                if cursor.position == *pos {
                    text.sections[0].value = label.clone();
                }
            }
            for (mut val, pos) in value_q.iter_mut() {
                if cursor.position == *pos {
                    val.0 = label.clone();
                }
            }
            cursor.position.col = (cursor.position.col + 1).min(5);
            // println!("Cursor {:?}", cursor.position);
        }
    }

    if keys.just_released(KeyCode::Return) && guess.word.len() == 5 {
        // println!("Text input: {}", guess.word);
        match game.make_guess_simple(&guess.word.to_lowercase()) {
            GameStatus::Ok(val) => {
                game.colors[cursor.position.row] = val;
                guess.word.clear();
                cursor.position.col = 0;
                cursor.position.row += 1;
            }
            GameStatus::InvalidWord => {
                println!("Invalid word");
            }
            GameStatus::GameOver => {
                println!("Game Over");
            }
            GameStatus::Victory(val) => {
                game.colors[cursor.position.row] = val;
                println!("VICTORY!");
            }
        }
        // println!("Cursor {:?}", cursor.position);
    }

    if keys.just_released(KeyCode::Back) {
        // println!("Removing last letter");
        guess.word.pop();
        if cursor.position.col != 0 {
            cursor.position.col -= 1;
        } else {
            cursor.position.col = 0;
        }
        for (mut text, pos) in text_q.iter_mut() {
            if cursor.position == *pos {
                text.sections[0].value.clear();
            }
        }
        for (mut val, pos) in value_q.iter_mut() {
            if cursor.position == *pos {
                val.0.clear();
            }
        }
        // println!("Cursor {:?}", cursor.position);
    }
}
