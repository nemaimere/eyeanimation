use bevy::prelude::*;

// Component to handle animation
#[derive(Component)]
struct AnimationTimer {
    timer: Timer,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, animate_sprite)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Load and create the texture atlas
    // let texture_handle = asset_server.load("C:/Users/nemai/work/eyeanimation/playpng/assets/blinktwocolumn.png");
     let texture_handle = asset_server.load("assets/dumb.png");

    println!("GOT THE HANDLE...\n");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(400.0, 400.0),  // Larger frame size
        1,                       // Fewer columns
        1,                       // Two rows
        None,                    // Optional padding
        None,                    // Optional offset
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    println!("GOT THE atlas HANDLE...\n");


    // Spawn the animated sprite
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_xyz(0.0, 0.0, 0.0)
                .with_scale(Vec3::splat(0.5)),  // Make the sprite smaller
            ..default()
        },
        AnimationTimer {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        },
    ));
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite)>,
) {
    for (mut timer, mut sprite) in &mut query {
        timer.timer.tick(time.delta());
        if timer.timer.just_finished() {
            sprite.index = (sprite.index + 1) % 4;  // Cycle through 4 frames
        }
    }
}
