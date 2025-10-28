use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Spawn a camera
    commands.spawn(Camera2d);

    // Load and display the image
    commands.spawn(Sprite::from_image(asset_server.load("dumb.png")));
}
