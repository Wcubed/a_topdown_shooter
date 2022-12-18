#![forbid(unsafe_code)]
#![warn(clippy::all, rust_2018_idioms)]

use a_topdown_shooter::MainPlugin;
use bevy::app::{App, PluginGroup};
use bevy::log::LogPlugin;
use bevy::DefaultPlugins;

fn main() -> color_eyre::eyre::Result<()> {
    // Install the fancy panic / error printer.
    color_eyre::install()?;

    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,a_topdown_shooter=debug".to_string(),
            level: bevy::log::Level::DEBUG,
        }))
        .add_plugin(MainPlugin)
        .run();

    Ok(())
}
