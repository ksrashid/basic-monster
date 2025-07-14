use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_mouse, move_to_target))
        .run();
}

#[derive(Component)]
struct Player {
    target_position: Vec2,
    player_speed: f32,
}

#[derive(Component)]
struct IsSelected;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            color: Color::srgb(0.8, 0.2, 0.2), // Red color
            custom_size: Some(Vec2::new(40.0, 40.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player {
            target_position: Vec2::ZERO,
            player_speed: 300.0,
        },
    ));
}



fn move_to_target(time: Res<Time>, mut query_player: Query<(&mut Transform, &Player)>) {
    for (mut transform, player) in query_player.iter_mut() {
        let direction = player.target_position - transform.translation.xy();
        let distance = direction.length();

        let move_player = direction.normalize_or_zero()
            * player.player_speed.clamp(0.0, distance)
            * time.delta_secs();
        transform.translation += move_player.extend(0.0);
    }
}

fn handle_mouse(
    mouse_button: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    query_camera: Single<(&Camera, &GlobalTransform)>,
    mut query_player: Query<&mut Player>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = query_camera.into_inner();
        if let Some(cursor_pos) = window.cursor_position() {
            match camera.viewport_to_world(camera_transform, cursor_pos) {
                Ok(world_pos) => {
                    let world_pos = world_pos.origin.truncate();
                    for mut player in query_player.iter_mut() {
                        player.target_position = world_pos;
                    }
                }
                Err(error) => {
                    warn!("Failed to get viewport from camera with: {error}");
                }
            }
        }
    }
}
