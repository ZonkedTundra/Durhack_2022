use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Race".to_string(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .add_system(horse_movement)
        .run();
}

#[derive(Component)]
pub struct Horse;

fn setup(mut commands: Commands) {
    // Create main camera
    commands.spawn_bundle(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: (ClearColorConfig::Custom(Color::rgb(0.18, 0.18, 0.18))).into(),
        },
        ..default()
    });

    for i in 1..10 {
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(-550., (75. * i as f32) - (60.), 1.),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(30., 30.)),
                    ..default()
                },
                ..default()
            })
            .insert(Horse);
    }
}

fn horse_movement(mut horse: Query<(&Horse, &mut Transform)>, keys: Res<Input<KeyCode>>) {
    let (horse, mut transform) = horse.single_mut();
    if keys.pressed(KeyCode::Right) {
        transform.translation.x += 5.;
    }
}
