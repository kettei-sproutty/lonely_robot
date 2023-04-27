use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn setup(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    if let Ok(window) = window_query.get_single() {
        let camera = Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
            ..default()
        };

        commands.spawn(camera);
    }
}
