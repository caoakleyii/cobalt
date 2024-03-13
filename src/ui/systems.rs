use bevy::{
    asset::AssetServer,
    ecs::{
        event::EventReader,
        query::With,
        system::{Commands, Res},
    },
    gizmos::gizmos::Gizmos,
    hierarchy::BuildChildren,
    math::{Vec2, Vec3},
    prelude::{Children, Query},
    render::{
        camera::{self, Camera, OrthographicProjection},
        color::Color,
        view::Visibility,
    },
    text::{Text, Text2dBundle, TextAlignment, TextStyle},
    transform::components::Transform,
    ui::{self, node_bundles::TextBundle, Style, UiRect},
    window::{PrimaryWindow, Window},
};
use bevy_health_bar::{ProgressBar, ProgressBarBundle};

use crate::{
    asset::{self, resources::AssetHandler},
    deck::components::{Graveyard, Hand, Library},
    input::components::PlayerCamera,
    player::{components::LocalPlayer, events::PlayerSpawnedEvent},
    stats::components::Health,
};

use super::components::DrawHandPrompt;

pub fn health_bar_update(
    query: Query<(&Health, &Children)>,
    mut bar_query: Query<&mut ProgressBar>,
) {
    for (health, children) in query.iter() {
        for &child in children.iter() {
            if let Ok(mut bar) = bar_query.get_mut(child) {
                bar.value = health.current;
            }
        }
    }
}

pub fn spawn_health_bar(
    mut reader_player_spawned: EventReader<PlayerSpawnedEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for player_spawned_event in reader_player_spawned.read() {
        let player_entity = player_spawned_event.entity;
        // Perhaps to move this to a generic system, when ever there's a new object w/ health.
        // but for now we only have players so will worry about it later.
        let transform = Transform::from_xyz(-15.0, 19.0, 0.0).with_scale(Vec3::new(0.5, 0.5, 0.5));
        commands
            .spawn(
                ProgressBarBundle::new(100.0, asset_server.load("ui/health_bar.png"))
                    .with_transform(transform),
            )
            .set_parent(player_entity);
    }
}

pub fn spawn_hud(
    mut reader_player_spawned: EventReader<PlayerSpawnedEvent>,
    mut commands: Commands,
    query_window: Query<&Window, With<PrimaryWindow>>,
    query_camera: Query<&OrthographicProjection, With<PlayerCamera>>,
    hud_items: Query<(&Library, &Hand, &Graveyard)>,
    asset_handler: Res<AssetHandler>,
) {
    for player_spawned_event in reader_player_spawned.read() {
        if !player_spawned_event.local_player {
            continue;
        }

        let player_entity = player_spawned_event.entity;

        let camera = query_camera.get_single().expect("No camera found.");

        let card_size = Vec2::new(40.0, 60.0);

        let bottom_padding = 40.0;

        let ui_font = asset_handler
            .fonts
            .get("default")
            .expect("default font has not be retrieved");

        let text_style = TextStyle {
            font: ui_font.font.clone(),
            font_size: 15.0,
            color: Color::WHITE,
        };
        let mut bundle = TextBundle::from_section("Press [F] to draw your hand", text_style)
            .with_style(Style {
                position_type: ui::PositionType::Relative,
                justify_self: ui::JustifySelf::Center,
                align_self: ui::AlignSelf::FlexEnd,
                margin: UiRect {
                    left: ui::Val::Px(0.0),
                    right: ui::Val::Px(0.0),
                    top: ui::Val::Px(0.0),
                    bottom: ui::Val::Px(card_size.y / camera.scale + bottom_padding),
                },
                ..Default::default()
            })
            .with_text_alignment(TextAlignment::Center);

        bundle.visibility = Visibility::Hidden;

        commands.spawn((bundle, DrawHandPrompt));
    }
}

pub fn draw_hud(
    hud_items: Query<(&Library, &Hand, &Graveyard), With<LocalPlayer>>,
    query_window: Query<&Window, With<PrimaryWindow>>,
    query_camera: Query<&OrthographicProjection, With<PlayerCamera>>,
    mut query_draw_hand_prompt: Query<&mut Visibility, With<DrawHandPrompt>>,
    mut gizmos: Gizmos,
) {
    for (library, hand, graveyard) in &hud_items {
        let camera = query_camera.get_single().expect("No camera found.");

        let window = query_window.get_single().expect("No window found.");

        let window_width = window.physical_width() as f32 * camera.scale;
        let window_height = window.physical_height() as f32 * camera.scale;

        let card_size = Vec2::new(40.0, 60.0);

        let bottom_padding = 10.0;
        let bottom_position = -(window_height / 2.0 - (card_size.y / 2.0 + bottom_padding));

        // Draw the library
        gizmos.rect_2d(
            Vec2::new(window_width / 2.0 - (card_size.x + 20.0), bottom_position),
            0.0,
            card_size.clone(),
            Color::PURPLE,
        );

        // Draw the hand
        let mut last_card_x = 0.0;
        if let Ok(mut draw_hand_prompt) = query_draw_hand_prompt.get_single_mut() {
            if hand.0.len() <= 0 {
                *draw_hand_prompt = Visibility::Visible;
            } else {
                *draw_hand_prompt = Visibility::Hidden;

                for i in 0..6 {
                    let position = Vec2::new(
                        window_width / 2.0
                    - ((card_size.x + 20.0) * 3.0) // spacing after the library
                    - ((card_size.x + 10.0) * i as f32), // spacing between cards
                        bottom_position,
                    );
                    last_card_x = position.x;
                    gizmos.rect_2d(position, 0.0, card_size.clone(), Color::WHITE);
                }
            }
        }

        // Draw the graveyard spot
        gizmos.rect_2d(
            Vec2::new(
                -(window_width / 2.0 - (card_size.x + 20.0)),
                bottom_position,
            ),
            0.0,
            card_size.clone(),
            Color::RED,
        );
    }
}
