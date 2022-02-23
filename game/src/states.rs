#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
	InGame,
	IncorrectWord,
	GameOver,
	Victory,
}
