# Building
Uses the lld linker on windows and macos, 
and the mold linker on linux.

Requires nightly rust, because we want to use `-Zshare-generics=y`
in the `.cargo/config.toml`.

Bevy is built dynamically linked, which means you need to deploy 
the bevy dll (or .so) along with the executable.