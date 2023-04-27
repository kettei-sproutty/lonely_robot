use crate::globals::Ui;
use bevy::a11y::accesskit::TextAlign;
use bevy::prelude::*;

pub struct ButtonComponent;

impl ButtonComponent {
    pub fn button_bundle() -> ButtonBundle {
        ButtonBundle {
            style: Style::default(),
            background_color: BackgroundColor(Color::NONE),
            ..default()
        }
    }

    pub fn text_bundle(text: &str, asset_server: &Res<AssetServer>) -> TextBundle {
        TextBundle {
            text: Text {
                sections: vec![TextSection::new(
                    text,
                    TextStyle {
                        color: Color::WHITE,
                        font_size: 32.,
                        font: asset_server.load("fonts/BrunoAce-Regular.ttf"),
                    },
                )],
                alignment: TextAlignment::Center,
                ..default()
            },
            ..default()
        }
    }
}
