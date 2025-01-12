use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
#[require(Transform)]
struct Player;
fn movement(
    mut query: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut vel = Vec2::new(0.0, 0.0);
    if keys.pressed(KeyCode::ArrowUp) {
        vel.y += 1.0;
    }
    if keys.pressed(KeyCode::ArrowDown) {
        vel.y -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowLeft) {
        vel.x -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowRight) {
        vel.x += 1.0;
    }
    if vel.length_squared() >= 0.01 {
        vel = vel.normalize();
    }
    vel *= time.delta_secs() * 600.0;
    for mut position in &mut query {
        position.translation += Vec3::new(vel.x, vel.y, 0.0);
    }
}

fn setup(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>) {
    let window = windows.get_single().unwrap();

    commands.spawn((
        Player,
        Transform::from_translation(Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0)),
        Sprite::from_color(Color::WHITE, Vec2::new(50.0, 50.0)),
    ));

    commands.spawn((
        Camera2d,
        Transform::from_translation(Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0)),
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, movement)
        .run();
}
