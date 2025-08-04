use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::view::{cam::mycam::MycamPlugin, ui::mainmenu::MenuPlugin};

pub mod cam;
pub mod ui;

pub struct ViewPlugins;

impl PluginGroup for ViewPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
        .add(MycamPlugin)
        .add(MenuPlugin)
    }
}