use crate::components::*;
use crate::consts::*;
use crate::resources::{CurrentGuess, Cursor, Game, GameStatus};
use crate::states::GameState;
use crate::utils::despawn_screen;
use bevy::input::keyboard::{KeyCode, KeyboardInput};
use bevy::input::ElementState;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.add_system_set(
			SystemSet::on_enter(GameState::Restarting)
				.with_system(despawn_screen::<Tile>.system())
				.with_system(restart_data),
		)
		.add_system_set(
			SystemSet::on_enter(GameState::InGame)
				.with_system(camera_setup)
				.with_system(setup),
		)
		.add_system_set(
			SystemSet::on_update(GameState::InGame)
				.with_system(tile_size_system)
				.with_system(tile_position_system)
				.with_system(tile_color_system)
				.with_system(keyboard_input)
				.with_system(gameover_check),
		);
	}
}

fn gameover_check(mut state: ResMut<State<GameState>>, game: Res<Game>) {
	if game.is_lost() {
		state.set(GameState::GameOver).unwrap();
	} else if game.is_won() {
		state.set(GameState::Victory).unwrap();
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
fn restart_data(
	mut state: ResMut<State<GameState>>,
	mut game: ResMut<Game>,
	mut cursor: ResMut<Cursor>,
	mut guess: ResMut<CurrentGuess>,
) {
	game.reset();
	cursor.position = TilePosition { row: 0, col: 0 };
	guess.word.clear();
	state.set(GameState::InGame).unwrap();
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
	mut keys: ResMut<Input<KeyCode>>,
	mut guess: ResMut<CurrentGuess>,
	mut cursor: ResMut<Cursor>,
	mut value_q: Query<(&mut Value, &TilePosition), With<Tile>>,
	mut text_q: Query<(&mut Text, &TilePosition), With<TextTileValue>>,
	mut game: ResMut<Game>,
	mut state: ResMut<State<GameState>>,
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
				// println!("Invalid word");
				state.push(GameState::IncorrectWord).unwrap();
			}
			GameStatus::GameOver => {
				// println!("Game Over");
				state.set(GameState::GameOver).unwrap();
			}
			GameStatus::Victory(val) => {
				game.colors[cursor.position.row] = val;
				// println!("VICTORY!");
				// state.set(GameState::Victory).unwrap();
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

	if keys.just_released(KeyCode::Escape) {
		state.set(GameState::Restarting).unwrap();
		keys.reset(KeyCode::Escape);
	}
}
