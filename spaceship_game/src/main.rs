mod debug;
mod movement;
mod spaceship;
mod camera;

use bevy::prelude::*;
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshiptPlugin;
use camera::CameraPlugin;

fn main() {
    App::new()
        //Bevy build-ins.
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })

        .add_plugins(DefaultPlugins)
        // User configured plugins
        .add_plugins(SpaceshiptPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .run();
}