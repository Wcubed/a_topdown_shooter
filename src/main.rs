#![forbid(unsafe_code)]
#![warn(clippy::all, rust_2018_idioms)]

use bevy::app::App;
use space_bees::MainPlugin;

fn main() -> color_eyre::eyre::Result<()> {
    // Install the fancy panic / error printer.
    color_eyre::install()?;

    App::new().add_plugin(MainPlugin).run();

    Ok(())
}
