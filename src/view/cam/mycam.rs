use bevy::prelude::*;


pub struct MycamPlugin;

impl Plugin for MycamPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup)
        ;
    }
}

fn setup(mut cmd:Commands) {
    cmd.spawn((
        Camera3d::default()
    ));
}


