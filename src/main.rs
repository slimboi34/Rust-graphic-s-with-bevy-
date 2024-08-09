use bevy::prelude::*;
use bevy::input::mouse::{MouseButtonInput, MouseMotion};
use bevy::window::PrimaryWindow; // Use the correct PrimaryWindow type

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // Adds default plugins like window, rendering, etc.
        .add_startup_system(setup) // Runs once when the app starts
        .add_system(ball_movement) // System to handle ball movement with the mouse
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
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 0.5, sectors: 32, stacks: 16 })),
            material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()), // Blue color
            transform: Transform::from_xyz(0.0, 0.5, 0.0), // Position the ball above the floor
            ..Default::default()
        },
        Ball, // Add a marker component to identify the ball entity
    ));
}

// Marker component to identify the ball
#[derive(Component)]
struct Ball;

// System to handle ball movement with the mouse
fn ball_movement(
    mut ball_query: Query<(&Ball, &mut Transform)>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    windows: Query<&Window, With<PrimaryWindow>>, // Use PrimaryWindow for accessing the main window
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let window = windows.single(); // Correct way to get the primary window

    if let Some(cursor_position) = window.cursor_position() {
        for (_, mut transform) in ball_query.iter_mut() {
            for event in mouse_button_input_events.iter() {
                if event.button == MouseButton::Left && event.state.is_pressed() {
                    // Convert the cursor position to 3D world coordinates
                    let (camera, camera_transform) = camera_query.single();
                    let cursor_ndc = Vec2::new(
                        (cursor_position.x / window.width()) * 2.0 - 1.0,
                        (cursor_position.y / window.height()) * 2.0 - 1.0,
                    );
                    let cursor_position = camera.ndc_to_world(camera_transform, cursor_ndc.extend(-1.0));
                    if let Some(world_pos) = cursor_position {
                        transform.translation.x = world_pos.x;
                        transform.translation.y = world_pos.y;
                    }
                }
            }

            for motion in mouse_motion_events.iter() {
                if window.cursor_position().is_some() {
                    // Move the ball with the mouse drag
                    transform.translation.x += motion.delta.x * 0.01;
                    transform.translation.y += motion.delta.y * -0.01;
                }
            }
        }
    }
}
