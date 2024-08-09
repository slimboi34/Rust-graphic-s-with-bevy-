use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // Adds default plugins like window, rendering, etc.
        .add_startup_systems((setup,)) // Runs once when the app starts
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // Set up the camera
    commands.spawn(Camera2dBundle::default());

    // Create the floor (red rectangle)
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.0, 0.0, 0.0), // Red color
            custom_size: Some(Vec2::new(500.0, 100.0)), // Size of the floor
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, -50.0, 0.0), // Position the floor
        ..Default::default()
    });

    // Create the surrounding (gray rectangle)
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.5, 0.5, 0.5), // Gray color
            custom_size: Some(Vec2::new(700.0, 400.0)), // Size of the surrounding
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, -1.0), // Position behind the floor and ball
        ..Default::default()
    });

    // Create the ball (blue circle)
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.0, 0.0, 1.0), // Blue color
            custom_size: Some(Vec2::new(50.0, 50.0)), // Size of the ball
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.1), // Position the ball above the floor
        ..Default::default()
    });
}
