#![forbid(unsafe_code)]
#![warn(clippy::all)]

mod camera;
mod input;
mod localization;

use crate::camera::GameCameraPlugin;
use crate::input::{Action, ActionRes, GlobalCursorPosition, InputPlugin};
use crate::localization::{Localization, LocalizationAssets, LocalizationPlugin};
use bevy::log::LogPlugin;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use tracing::info;

const GREETING_ID: &str = "greeting";

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        // Enable debug logging only when running in debug mode.
        #[cfg(debug_assertions)]
        app.add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    level: bevy::log::Level::DEBUG,
                    filter: "info,wgpu_core=warn,wgpu_hal=warn,a_topdown_shooter=debug".into(),
                })
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                }),
        );

        #[cfg(not(debug_assertions))]
        app.add_plugins(DefaultPlugins.set(LogPlugin {
            level: bevy::log::Level::INFO,
            filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
        }));

        app.add_plugin(InputPlugin)
            .add_plugin(LocalizationPlugin)
            .add_plugin(GameCameraPlugin)
            .add_plugin(EguiPlugin)
            .add_loading_state(
                LoadingState::new(GameState::AssetLoading)
                    .continue_to_state(GameState::Main)
                    .with_dynamic_collections::<StandardDynamicAssetCollection>(vec![
                        "dynamic_assets.assets",
                    ])
                    .with_collection::<ImageAssets>()
                    .with_collection::<LocalizationAssets>(),
            )
            .add_state(GameState::AssetLoading)
            .add_system_set(SystemSet::on_enter(GameState::Main).with_system(spawn_planets))
            .add_system_set(
                SystemSet::on_update(GameState::Main)
                    .with_system(greeting_system)
                    .with_system(planet_selection_system)
                    .with_system(planet_info_ui),
            );
    }
}

#[derive(AssetCollection, Resource)]
struct ImageAssets {
    #[asset(key = "image.planet")]
    planet: Handle<Image>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    AssetLoading,
    Main,
}

#[derive(Component)]
struct Planet {
    name: String,
}

#[derive(Component)]
struct Selected;

fn spawn_planets(mut commands: Commands, image_assets: Res<ImageAssets>) {
    for index in 0..10 {
        commands
            .spawn(SpriteBundle {
                texture: image_assets.planet.clone(),
                transform: Transform::from_xyz(index as f32 * 100.0, 0.0, 0.0),
                ..default()
            })
            .insert(Planet {
                name: format!("Planet {index}"),
            });
    }
}

fn planet_selection_system(
    mut commands: Commands,
    actions: ActionRes,
    cursor_position: Res<GlobalCursorPosition>,
    planets: Query<(Entity, &GlobalTransform, Option<&Selected>), With<Planet>>,
) {
    if actions.just_pressed(Action::PrimaryInteraction) {
        let mut found_planet = false;

        for (entity, transform, maybe_selected) in planets.iter() {
            // TODO (Wybe 2022-12-30): Put in planet sizes.
            if cursor_position.distance(transform.translation().xy()) < 16.0 {
                found_planet = true;

                if maybe_selected.is_none() {
                    commands.entity(entity).insert(Selected);
                } else {
                    commands.entity(entity).remove::<Selected>();
                }
            }
        }

        if !found_planet {
            // No planets found? Deselect all.
            for (entity, _, maybe_selected) in planets.iter() {
                if maybe_selected.is_some() {
                    commands.entity(entity).remove::<Selected>();
                }
            }
        }
    }
}

fn planet_info_ui(
    mut egui_context: ResMut<EguiContext>,
    selected_planets: Query<&Planet, With<Selected>>,
) {
    egui::Window::new("Selected planets")
        .anchor(egui::Align2::LEFT_TOP, [0., 0.])
        .show(egui_context.ctx_mut(), |ui| {
            for planet in selected_planets.iter() {
                ui.label(&planet.name);
            }
        });
}

fn greeting_system(actions: ActionRes, localization: Res<Localization>) {
    if actions.just_pressed(Action::HelloAction) {
        info!("{}", localization.localize(GREETING_ID));
    }
}
