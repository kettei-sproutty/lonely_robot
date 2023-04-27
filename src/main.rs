#![feature(inherent_associated_types)]

use bevy::prelude::*;

mod camera;
mod globals;
mod menu;
mod states;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (globals::WINDOW_WIDTH, globals::WINDOW_HEIGHT).into(),
                        title: globals::APP_NAME.to_owned(),
                        resizable: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_state::<states::AppState>()
        .add_plugin(menu::MenuPlugin)
        .add_startup_system(camera::setup)
        .run();
}
