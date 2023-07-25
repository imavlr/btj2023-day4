use bevy::prelude::*;

pub fn move_towards(from: Vec2, to: Vec2, max_distance: f32) -> Vec2 {
    let to_target = to - from;
    let total_distance = to_target.length();
    if total_distance <= max_distance {
        return to;
    }
    return from + (to_target / total_distance) * max_distance;
}
