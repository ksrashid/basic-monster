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

#[derive(Component)]
struct IsMoving;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            color: Color::srgb(0.8, 0.2, 0.2),
            custom_size: Some(Vec2::new(40.0, 40.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player {
            target_position: Vec2::ZERO,
            player_speed: 300.0,
        },
    ));

 commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.8, 0.2), 
            custom_size: Some(Vec2::new(40.0, 40.0)),
            ..default()
        },
        Transform::from_xyz(50.0, 50.0, 0.0),
        Player {
            target_position: Vec2::ZERO,
            player_speed: 300.0,
        },
    ));
}



fn move_to_target(time: Res<Time>, mut query_player: Query<(&mut Transform, &Player), With<IsSelected>>, ) {
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
    mut query_player_mut: Query<&mut Player>,
    mut commands: Commands,
    query_player: Query<(Entity, &Transform), With<Player>>,
    query_selected: Query<Entity, With<IsSelected>>,
    query_moving: Query<Entity, With<IsMoving>>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = query_camera.into_inner();
        if let Some(cursor_pos) = window.cursor_position() {
            match camera.viewport_to_world(camera_transform, cursor_pos) {
                Ok(world_pos) => {
                    let world_pos = world_pos.origin.truncate();
                    let grid_size = 50.0;
                    
                    let chosen_grid_pos = world_to_grid(world_pos, grid_size);
                    
                    let mut chosen_player = None;
                    
                    for (entity, transform) in query_player.iter() {
                        let chosen_player_pos = world_to_grid(transform.translation.truncate(), grid_size);
                        if chosen_player_pos == chosen_grid_pos {
                            chosen_player = Some(entity);
                            break;
                        }
                    }
    

                    if let Some(entity) = chosen_player {
                        let 
                        for selected_entity in query_selected.iter() {
                            commands.entity(selected_entity).remove::<IsSelected>();
                        }                        
                        commands.entity(entity).insert(IsSelected);

                    }



                    else {
                        // No player at     clicked position - try to move selected player
                        let selected_players: Vec<Entity> = query_selected.iter().collect();
                        
                        if selected_players.len() == 1 {
                            let selected_entity = selected_players[0];
                            if let Ok(mut player) = query_player_mut.get_mut(selected_entity) {
                                let target_world_pos = grid_to_world(chosen_grid_pos, grid_size);
                                player.target_position = target_world_pos;
                            }
                        }
                    }
                }
                Err(error) => {
                    warn!("Failed to get viewport from camera with: {error}");
                }
            }
        }
    }
}

fn world_to_grid(world_pos: Vec2, grid_size: f32) -> Vec2 {
    Vec2::new(
        (world_pos.x / grid_size).round(),
        (world_pos.y / grid_size).round(),
    )
}

// Helper function to convert grid coordinates to world position (center of cell)
fn grid_to_world(grid_pos: Vec2, grid_size: f32) -> Vec2 {
    Vec2::new(
        grid_pos.x * grid_size + grid_size * 0.5,
        grid_pos.y * grid_size + grid_size * 0.5,
    )
}   
