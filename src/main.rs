mod dice;
mod player;
mod view;

use bevy::prelude::*;
use player::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, view::setup)
        .run();
}
