#![forbid(unsafe_code)]
#![warn(clippy::all)]

mod camera;
mod input;
mod localization;

use crate::camera::GameCameraPlugin;
use crate::input::{Action, ActionRes, InputPlugin};
use crate::localization::{Localization, LocalizationAssets, LocalizationPlugin};
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use tracing::info;

const GREETING_ID: &str = "greeting";

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        // Enable debug logging only when running in debug mode.
        #[cfg(debug_assertions)]
        app.add_plugins(DefaultPlugins.set(LogPlugin {
            level: bevy::log::Level::DEBUG,
            filter: "info,wgpu_core=warn,wgpu_hal=warn,a_topdown_shooter=debug".into(),
        }));

        #[cfg(not(debug_assertions))]
        app.add_plugins(DefaultPlugins.set(LogPlugin {
            level: bevy::log::Level::INFO,
            filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
        }));

        app.add_plugin(InputPlugin)
            .add_plugin(LocalizationPlugin)
            .add_plugin(GameCameraPlugin)
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
            .add_system_set(SystemSet::on_enter(GameState::Main).with_system(spawn_player))
            .add_system_set(SystemSet::on_update(GameState::Main).with_system(greeting_system));
    }
}

#[derive(AssetCollection, Resource)]
struct ImageAssets {
    #[asset(key = "image.player")]
    player: Handle<Image>,
}

#[derive(Component)]
struct Player;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    AssetLoading,
    Main,
}

fn spawn_player(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands
        .spawn(SpriteBundle {
            texture: image_assets.player.clone(),
            ..default()
        })
        .insert(Player);
}

fn greeting_system(actions: ActionRes, localization: Res<Localization>) {
    if actions.just_pressed(Action::HelloAction) {
        info!("{}", localization.localize(GREETING_ID));
    }
}
