use bevy::prelude::*;

use crate::states::AppState;

mod components;
mod systems;

#[derive(Component)]
pub struct Menu;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(systems::spawn_menu.in_schedule(OnEnter(AppState::Menu)))
            .add_system(systems::despawn_menu.in_schedule(OnExit(AppState::Menu)));
    }
}
