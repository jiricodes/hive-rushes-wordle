#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
	Restarting,
	InGame,
	IncorrectWord,
	GameOver,
	Victory,
}
