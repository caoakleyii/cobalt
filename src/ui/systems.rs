use bevy::{
    asset::AssetServer,
    ecs::{
        entity::Entity,
        event::EventReader,
        query::{With, Without},
        system::{Commands, Res},
    },
    hierarchy::BuildChildren,
    math::{Vec2, Vec3},
    prelude::{Children, Query},
    render::{camera::OrthographicProjection, color::Color, view::Visibility},
    text::{TextAlignment, TextStyle},
    transform::components::Transform,
    ui::{
        self,
        node_bundles::{ImageBundle, NodeBundle, TextBundle},
        *,
    },
};
use bevy_health_bar::{ProgressBar, ProgressBarBundle};

use crate::{
    asset::{enums::Sprites, resources::AssetHandler},
    deck::{
        card::components::Flipped,
        components::{Graveyard, Hand, Library},
        events::CardDrawnEvent,
    },
    input::components::PlayerCamera,
    player::{components::LocalPlayer, events::EntitySpawnedEvent},
    stats::components::Health,
};

use super::components::{CardBack, CardFront, CardText, CardUI, DrawHandPrompt, HandUI, LibraryUI};

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
    mut reader_entity_spawned: EventReader<EntitySpawnedEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for player_spawned_event in reader_entity_spawned.read() {
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
    mut reader_entity_spawned: EventReader<EntitySpawnedEvent>,
    mut commands: Commands,
    query_camera: Query<&OrthographicProjection, With<PlayerCamera>>,
    is_local_player: Query<With<LocalPlayer>>,
    hud_items: Query<(&Library, &Hand, &Graveyard)>,
    asset_handler: Res<AssetHandler>,
) {
    for player_spawned_event in reader_entity_spawned.read() {
        if !is_local_player.contains(player_spawned_event.entity) {
            continue;
        }

        let player_entity = player_spawned_event.entity;

        let camera = query_camera.get_single().expect("No camera found.");

        let card_size = Vec2::new(52.0, 60.0);
        let padding = 40.0;

        let ui_font = asset_handler
            .fonts
            .get("default")
            .expect("default font has not be retrieved");

        let card_back_image = asset_handler
            .textures
            .get(&Sprites::CardBack)
            .expect("Card back image not found.");

        let card_front_image = asset_handler
            .textures
            .get(&Sprites::CardFront)
            .expect("Card front image not found.");

        let (library, _hand, _graveyard) = hud_items
            .get(player_entity)
            .expect(format!("No hud items found for {:?}", player_entity).as_str());

        // DRAW HAND UI
        let height = card_size.y / camera.scale + padding;
        let width = card_size.y / camera.scale + padding;

        let hand_ui = NodeBundle {
            style: Style {
                width: Val::Percent(70.),
                height: Val::Px(height),
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::FlexEnd,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        };
        commands.spawn((hand_ui, HandUI)).with_children(|parent| {
            let text_style = TextStyle {
                font: ui_font.font.clone(),
                font_size: 15.0,
                color: Color::WHITE,
            };
            let mut empty_hand_text_bundle =
                TextBundle::from_section("Press [R] to draw your hand", text_style)
                    .with_style(Style {
                        justify_self: ui::JustifySelf::Center,
                        align_self: ui::AlignSelf::Center,
                        ..Default::default()
                    })
                    .with_text_alignment(TextAlignment::Center);

            empty_hand_text_bundle.visibility = Visibility::Visible;

            parent.spawn((empty_hand_text_bundle, DrawHandPrompt));
        });

        // DRAW LIBRARY UI
        let library_ui = NodeBundle {
            style: Style {
                width: Val::Percent(15.),
                height: Val::Px(height),
                justify_self: JustifySelf::End,
                align_self: AlignSelf::FlexEnd,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        };
        let library_ui_entity = commands.spawn((library_ui, LibraryUI)).id();

        for card_entity in library.0.iter() {
            if let Some(mut entity_command) = commands.get_entity(card_entity.entity) {
                // <DIV>
                entity_command
                    .insert((
                        NodeBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                width: Val::Px(width / 2.),
                                height: Val::Px(height / 2.),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        CardUI,
                        Flipped::Down,
                    ))
                    .with_children(|parent| {
                        // <Card Back IMG>
                        parent.spawn((
                            ImageBundle {
                                image: UiImage::new(card_back_image.texture_atlas.texture.clone()),
                                style: Style {
                                    width: Val::Percent(100.),
                                    justify_content: JustifyContent::Center,
                                    justify_items: JustifyItems::Center,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            CardBack,
                        ));
                        // <Card Front IMG>
                        parent
                            .spawn((
                                ImageBundle {
                                    image: UiImage::new(
                                        card_front_image.texture_atlas.texture.clone(),
                                    ),
                                    style: Style {
                                        width: Val::Percent(100.),
                                        padding: UiRect::top(Val::Px(10.0)),
                                        justify_content: JustifyContent::Center,
                                        justify_items: JustifyItems::Center,
                                        display: Display::None,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                CardFront,
                            ))
                            .with_children(|card_parent| {
                                // <TEXT>
                                let text_style = TextStyle {
                                    font: ui_font.font.clone(),
                                    font_size: 13.0,
                                    color: Color::hex("#000000").unwrap(),
                                };
                                card_parent.spawn((
                                    TextBundle::from_section(
                                        format!("{}", card_entity.card.name),
                                        text_style,
                                    ),
                                    CardText,
                                ));
                            });
                    });

                entity_command.set_parent(library_ui_entity);
            }
        }

        let graveyard_ui = NodeBundle {
            style: Style {
                width: Val::Percent(15.),
                height: Val::Px(height),
                position_type: PositionType::Relative,
                justify_self: JustifySelf::Start,
                align_self: AlignSelf::FlexEnd,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        };

        let _graveyard_ui_entity = commands.spawn(graveyard_ui).id();
    }
}

pub fn card_drawn(
    mut reader_card_drawn: EventReader<CardDrawnEvent>,
    mut commands: Commands,
    mut card_ui_node_query: Query<&mut Style, With<CardUI>>,
    hand_ui_query: Query<Entity, With<HandUI>>,
) {
    for card_drawn_event in reader_card_drawn.read() {
        if let Ok(hand_ui_entity) = hand_ui_query.get_single() {
            println!("Draw card");
            if let Some(mut entity_command) = commands.get_entity(card_drawn_event.card.entity) {
                entity_command.set_parent(hand_ui_entity);
                entity_command.insert(Flipped::Up);
            }

            if let Ok(mut style) = card_ui_node_query.get_mut(card_drawn_event.card.entity) {
                style.position_type = PositionType::Relative;
            }
        }
    }
}

pub fn flip_card(
    mut q_card_front: Query<&mut Style, (With<CardFront>, Without<CardBack>)>,
    mut q_card_back: Query<&mut Style, (With<CardBack>, Without<CardFront>)>,
    q_flip_state: Query<(&Flipped, &Children)>,
) {
    for (flipped, children) in q_flip_state.iter() {
        for &child in children.iter() {
            if let Ok(mut card_back_style) = q_card_back.get_mut(child) {
                card_back_style.display = match flipped {
                    Flipped::Up => Display::None,
                    Flipped::Down => Display::Flex,
                };
            }

            if let Ok(mut card_front_style) = q_card_front.get_mut(child) {
                card_front_style.display = match flipped {
                    Flipped::Up => Display::Flex,
                    Flipped::Down => Display::None,
                };
            }
        }
    }
}
pub fn draw_hand_prompt(
    hand_query: Query<&Hand, With<LocalPlayer>>,
    mut draw_hand_prompt_query: Query<&mut Style, With<DrawHandPrompt>>,
) {
    if let Ok(hand) = hand_query.get_single() {
        let display = if hand.0.len() <= 0 {
            Display::Flex
        } else {
            Display::None
        };

        if let Ok(mut draw_hand_prompt_style) = draw_hand_prompt_query.get_single_mut() {
            draw_hand_prompt_style.display = display;
        }
    }
}
