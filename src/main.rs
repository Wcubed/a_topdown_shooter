#![forbid(unsafe_code)]
#![warn(clippy::all, rust_2018_idioms)]

use a_topdown_shooter::MainPlugin;
use bevy::app::App;
use bevy::DefaultPlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MainPlugin)
        .run();
}
