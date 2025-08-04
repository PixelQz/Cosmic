use bevy::prelude::*;
#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]  // 这将是初始状态
    MainMenu,
    Loading,
    InGame,
    Paused,
    GameOver,
}
