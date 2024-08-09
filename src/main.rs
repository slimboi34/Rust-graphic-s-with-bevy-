use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // Adds default plugins like window, rendering, etc.
        .add_systems(Startup, setup) // Runs once when the app starts
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Set up the camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // Add a light source
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 3000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // Create the floor (red plane)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0, subdivisions: 1 })),
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()), // Red color
        transform: Transform::from_xyz(0.0, 0.0, 0.0), // Position the floor
        ..Default::default()
    });

    // Create the surrounding (gray plane)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0, subdivisions: 1 })),
        material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()), // Gray color
        transform: Transform::from_xyz(0.0, -0.01, -5.0).with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..Default::default()
    });

    // Create the ball (blue sphere)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 0.5, sectors: 32, stacks: 16 })),
        material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()), // Blue color
        transform: Transform::from_xyz(0.0, 0.5, 0.0), // Position the ball above the floor
        ..Default::default()
    });
}
