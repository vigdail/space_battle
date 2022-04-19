#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub enum GameState {
    Loading,
    MainMenu,
    Countdown,
    Gameplay,
    GameOver,
}
