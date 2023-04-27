use bevy::prelude::Color;
use bevy::ui::Val;

pub const WINDOW_WIDTH: f32 = 1280.;
pub const WINDOW_HEIGHT: f32 = 720.;

pub const APP_NAME: &str = "Lonely Robot";

pub struct Ui;

impl Ui {
    pub type ButtonSize = (f32, f32);

    pub const COLOR_TRANSPARENT: Color = Color::NONE;
    pub const BUTTON_SIZE_LARGE: (Val, Val) = (Val::Px(120.), Val::Px(40.));
}
