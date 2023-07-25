use bevy::prelude::*;

pub struct DespawnAfterPlugin;

impl Plugin for DespawnAfterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, despawn_after);
    }
}

#[derive(Component)]
pub struct DespawnAfter {
    pub time_to_destroy: f32,
}

fn despawn_after(mut commands: Commands, time: Res<Time>, q_des: Query<(Entity, &DespawnAfter)>) {
    for (e, d) in q_des.iter() {
        if d.time_to_destroy <= time.elapsed_seconds() {
            commands.entity(e).despawn();
        }
    }
}
