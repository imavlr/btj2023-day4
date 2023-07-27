use bevy::math::Vec3Swizzles;
use bevy::prelude::*;

use crate::menu::LastActivity;
use crate::player::Player;
use crate::utils::move_towards;

#[derive(Component)]
pub struct MoveTarget {
    pub target: Option<Vec2>,
}

#[derive(Component)]
pub struct MoveDirection(pub Vec2);

#[derive(Component)]
pub struct MoveSpeed(pub f32);

pub fn move_targets(
    time: Res<Time>,
    mut q_moving: Query<(&mut Transform, &mut MoveTarget, &MoveSpeed)>,
) {
    for (mut transform, mut target, speed) in q_moving.iter_mut() {
        if match target.target {
            Some(target) => {
                let position = move_towards(
                    transform.translation.xy(),
                    target,
                    speed.0 * time.delta_seconds(),
                );
                transform.translation = position.extend(2f32);
                transform.translation.xy().distance_squared(target) <= 0.1f32
            }
            None => false,
        } {
            target.target = None;
        }
    }
}

pub fn move_direction(
    time: Res<Time>,
    mut q_moving: Query<(&mut Transform, &MoveDirection, &MoveSpeed)>,
) {
    for (mut transform, move_direction, speed) in q_moving.iter_mut() {
        transform.translation += (move_direction.0 * speed.0 * time.delta_seconds()).extend(0f32);
    }
}

pub fn wasd_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut q_moving: Query<(&mut MoveDirection, &MoveSpeed), With<Player>>,
    mut last_activity: ResMut<LastActivity>,
) {
    for (mut move_direction, speed) in q_moving.iter_mut() {
        let mut direction = Vec2::ZERO;
        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            direction += Vec2::Y;
        }
        if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            direction -= Vec2::Y;
        }
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            direction -= Vec2::X;
        }
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            direction += Vec2::X;
        }
        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }
        if direction != Vec2::ZERO {
            last_activity.0.reset();
        }
        move_direction.0 = direction;
    }
}
