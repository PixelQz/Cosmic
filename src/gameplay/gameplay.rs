use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct GamePlayPlugin;

// impl PluginGroup for GamePlayPlugin {
//     fn build(self) -> bevy::app::PluginGroupBuilder {
//         PluginGroupBuilder::start::<Self>()
//         .add(plugin)
//     }
// }

impl Plugin for GamePlayPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup)
        ;
    }
}

fn setup() {
    
}