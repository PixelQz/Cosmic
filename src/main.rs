use  bevy::prelude::*;

use crate::view::cam::mycam::MycamPlugin;

mod state;
mod view;
mod gameplay;
mod scence;
use view::ViewPlugins;
use state::GameState;
fn main() {
    let mut app =App::new();
    app
   
    .add_plugins(DefaultPlugins)
    // .add_plugins(MycamPlugin)
    .init_state::<GameState>()
    .add_plugins(ViewPlugins)
    ;
    app.run();
}