#![forbid(unsafe_code)]
#![warn(clippy::all, rust_2018_idioms)]

use a_topdown_shooter::MainPlugin;
use bevy::app::App;

fn main() -> color_eyre::eyre::Result<()> {
    // Install the fancy panic / error printer.
    color_eyre::install()?;

    App::new().add_plugin(MainPlugin).run();

    Ok(())
}
