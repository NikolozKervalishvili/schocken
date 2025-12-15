mod dice;
mod middleware;
mod player;
mod schockerr;
mod server;
mod view;

use bevy::prelude::*;
use player::*;

// bevy should listen to a receiver, tokio has the sender
// fn receive_tokio()

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, view::setup)
        .run();
}
