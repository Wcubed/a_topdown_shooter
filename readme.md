# Building

To run, use [cargo-make](https://github.com/sagiegurari/cargo-make) (`cargo make run`)

Or if you want to run `cargo run` with something else (for example, your IDE):
convert the svg's to png's by running `cargo make export-svg`

## Dependencies
- `cargo-make` for building
- `inkscape` for converting the `.svg` assets into `.png` images.
- Uses the lld linker on windows and macos, and the mold linker on linux.

Requires nightly rust, because we want to use `-Zshare-generics=y`
in the `.cargo/config.toml`.

Bevy is built dynamically linked, which means you need to deploy 
the bevy dll (or .so) along with the executable.