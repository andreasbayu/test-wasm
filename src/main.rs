use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Update, movement)
        .add_systems(Startup, setup)
        .add_plugins(DefaultPlugins)
        .run();
}

#[derive(Component)]
struct Movable;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // base
    commands.spawn((PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(20.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    }, Movable));

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule { radius: 0.25, depth: 0.5, rings: 6, ..default() })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.9, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));

    // instructions
    commands.spawn(
        TextBundle::from_section(
            "Use wasd or arrow keys to move object",
            TextStyle {
                font_size: 22.0,
                ..default()
            },
        )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(12.0),
                left: Val::Px(12.0),
                ..default()
            }),
    );
}

fn movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Movable>>,
) {
    for mut transform in &mut query {
        let mut direction = Vec3::ZERO;
        if input.pressed(KeyCode::Up) || input.pressed(KeyCode::W) {
            direction.z += 1.0;
        }
        if input.pressed(KeyCode::Down) || input.pressed(KeyCode::S) {
            direction.z -= 1.0;
        }
        if input.pressed(KeyCode::Left) || input.pressed(KeyCode::A) {
            direction.x += 1.0;
        }
        if input.pressed(KeyCode::Right) || input.pressed(KeyCode::D) {
            direction.x -= 1.0;
        }

        transform.translation += time.delta_seconds() * 2.0 * direction;
    }
}
