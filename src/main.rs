use bevy::prelude::*;
use std::fmt::Debug;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .init_resource::<MyWorldCoords>()
        //.insert_resource(MyWorldCoords(Vec2::new(50.0, 75.0)))
        .add_systems(Update, move_to_target)
        .run();
}

#[derive(Component)]
struct Player {
    player_speed: f32,
}

#[derive(Resource, Default)]
struct MyWorldCoords(Vec2);

#[derive(Component)]
struct IsSelected;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Sprite {
                color: Color::srgb(0.8, 0.2, 0.2),
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
            Pickable::default(),
            Player {
                player_speed: 300.0,
            },
        ))
        .observe(recolor_on::<Pointer<Over>>(Color::srgb(0.0, 0.8, 0.2)))
        .observe(recolor_on::<Pointer<Out>>(Color::srgb(1.0, 1.0, 0.0)))
        .observe(select_player::<Pointer<Click>>());
}

fn recolor_on<E: Debug + Clone + Reflect>(color: Color) -> impl Fn(Trigger<E>, Query<&mut Sprite>) {
    move |ev, mut sprites| {
        let Ok(mut sprite) = sprites.get_mut(ev.target()) else {
            return;
        };
        sprite.color = color;
    }
}

fn select_player<E: Debug + Clone + Reflect>()
-> impl Fn(Trigger<E>, Commands, Query<Option<&IsSelected>, With<Player>>) {
    move |ev, mut commands, query_player| {
        if let Ok(is_selected) = query_player.get(ev.target()) {
            match is_selected {
                Some(_) => {
                    commands.entity(ev.target()).remove::<IsSelected>();
                    println!("unselected");
                }
                None => {
                    commands.entity(ev.target()).insert(IsSelected);
                    println!("selected");
                }
            }
        }
    }
}

fn move_to_target(
    mycoords: Res<MyWorldCoords>,
    mut query_player: Query<(&mut Transform, &Player), With<IsSelected>>,
    time: Res<Time>,
) {
    for (mut transform, player) in query_player.iter_mut() {
        let direction = mycoords.0 - transform.translation.xy();
        let distance = direction.length();

        let move_player = direction.normalize_or_zero()
            * player.player_speed.clamp(0.0, distance)
            * time.delta_secs();
        transform.translation += move_player.extend(0.0);
    }
}
