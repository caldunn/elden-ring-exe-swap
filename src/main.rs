#![feature(fs_try_exists)]
use std::{
    env,
    fs::{rename, try_exists},
    io,
};

#[non_exhaustive]
struct Mode;

impl Mode {
    pub const SEAMLESS: &str = ".seamless";
    pub const VANILLA: &str = ".vanilla";
}

fn main() {
    // get home
    let home = env::var("HOME").expect("Unable to read $HOME env variable");

    let root = format!(
        "{}/.steam/steam/steamapps/common/ELDEN RING/Game/start_protected_game.exe",
        home
    );

    try_exists(format!("{}{}", root, Mode::VANILLA))
        .map(|seamless_loaded| {
            match seamless_loaded {
                true => swap(&root, Mode::SEAMLESS, Mode::VANILLA),
                false => swap(&root, Mode::VANILLA, Mode::SEAMLESS),
            }
            .expect("Couldn't rename file")
        })
        .expect("Couldn't read game directory");
}

fn swap(root: &str, de: &str, active: &str) -> io::Result<()> {
    // rename current -> rename switch.
    println!(
        "Switching from\x1b[1m {}\x1b[22m to \x1b[1m{}",
        &de[1..],
        &active[1..]
    );
    let deactivate = format!("{}{}", root, de);

    let activate = format!("{}{}", root, active);

    rename(root, deactivate)?;
    rename(activate, root)?;

    Ok(())
}
