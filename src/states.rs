use bevy::prelude::*;

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq)]
pub enum AppState {
    #[default]
    Menu,
    Simulation,
}

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq)]
pub enum SimulationState {
    #[default]
    None,
    Play,
    Pause,
}
