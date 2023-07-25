use bevy::{math::Vec3Swizzles, prelude::*, window::PrimaryWindow};

use crate::{bullets::CommandsSpawnBullet, movement::MoveTarget, Cooldown, TeamIdx};

#[derive(Component, Debug)]
pub struct Player;

pub fn handle_mouse_to_move(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    mut q_moves: Query<&mut MoveTarget, With<Player>>,
    camera: Query<(&GlobalTransform, &Camera)>,
) {
    if buttons.pressed(MouseButton::Left) {
        if let Some(position) = q_windows.single().cursor_position() {
            if let Some((camera_transform, camera)) = camera.iter().next() {
                let Some(position) = camera.viewport_to_world_2d(camera_transform, position) else {
                    return;
                };
                for mut m in q_moves.iter_mut() {
                    m.target = Some(position);
                }
            }
        }
    } else {
        for mut m in q_moves.iter_mut() {
            m.target = None;
        }
    }
}

pub fn handle_clicks_to_fire(
    mut commands: Commands,
    time: Res<Time>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    mut q_attackers: Query<(Entity, &Transform, &MoveTarget, &TeamIdx, &Cooldown), With<Player>>,
    camera: Query<(&GlobalTransform, &Camera)>,
) {
    if buttons.just_released(MouseButton::Left) {
        if let Some(position) = q_windows.single().cursor_position() {
            if let Some((camera_transform, camera)) = camera.iter().next() {
                let Some(position) = camera.viewport_to_world_2d(camera_transform, position) else {
                        return;
                    };
                for (entity, transform, _, team, cooldown) in q_attackers.iter_mut() {
                    let t_position = transform.translation.xy();
                    // TODO: rework bullet spawn to take place with an event
                    if commands
                        .spawn_bullet(
                            entity,
                            t_position,
                            (position - t_position).normalize_or_zero(),
                            team.clone(),
                            cooldown,
                            &time,
                        )
                        .is_ok()
                    {
                        commands.entity(entity).insert(Cooldown {
                            start_time: time.elapsed_seconds(),
                            duration: cooldown.duration,
                        });
                    }
                }
            }
        }
    }
}
