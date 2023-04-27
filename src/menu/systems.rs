use bevy::prelude::*;
use bevy::ui::FlexDirection::Column;
use bevy::window::PrimaryWindow;

use crate::menu::components::button::ButtonComponent;
use crate::menu::Menu;

pub fn spawn_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let vertical_gap = 720. / window.height() * 20.;
    let padding_left = 1080. / window.width() * 150.;
    let padding_top = 720. / window.height() * 300.;

    commands
        .spawn(ImageBundle {
            image: UiImage::new(asset_server.load("images/particles.png")),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        background_color: BackgroundColor(Color::Lcha {
                            lightness: 0.,
                            chroma: 0.14,
                            hue: 0.289,
                            alpha: 0.74,
                        }),
                        style: Style {
                            position: UiRect {
                                left: Val::Px(0.),
                                top: Val::Px(0.),
                                ..default()
                            },
                            flex_direction: Column,
                            justify_content: JustifyContent::Start,
                            align_items: AlignItems::Start,
                            aspect_ratio: (16. / 9.).into(),
                            gap: Size::new(Val::Undefined, Val::Px(vertical_gap)),
                            padding: UiRect {
                                left: Val::Px(padding_left),
                                top: Val::Px(padding_top),
                                ..default()
                            },
                            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                            ..default()
                        },
                        ..default()
                    },
                    Menu,
                ))
                .with_children(|parent| {
                    // Title

                    // Menu Sections
                    let sections = vec![
                        "New Game", "Continue", "Options", "Credits", "Extras", "Exit",
                    ];
                    sections.iter().for_each(|button_text| {
                        parent
                            .spawn(ButtonComponent::button_bundle())
                            .with_children(|parent| {
                                parent.spawn(ButtonComponent::text_bundle(
                                    button_text,
                                    &asset_server,
                                ));
                            });
                    });
                });
        });
}

pub fn despawn_menu(mut commands: Commands, menu_entity_query: Query<Entity, With<Menu>>) {
    if let Ok(menu_entity) = menu_entity_query.get_single() {
        commands.entity(menu_entity).despawn_recursive();
    }
}
