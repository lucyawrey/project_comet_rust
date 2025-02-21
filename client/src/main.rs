use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use database::DatabasePlugin;
use debug::DebugPlugin;
use hello::HelloPlugin;
mod components;
mod database;
mod debug;
mod hello;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin))
        .add_plugins(DatabasePlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(HelloPlugin)
        .run();
}
