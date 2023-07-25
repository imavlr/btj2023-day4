use bevy::{ecs::system::Command, prelude::*};

use crate::{
    despawn_after::DespawnAfter,
    movement::{MoveDirection, MoveSpeed},
    Cooldown, RemoveOnRespawn, TeamIdx,
};

#[derive(Event, Debug)]
pub struct EventBulletSpawn {
    pub origin: Vec2,
}

pub struct SpawnBulletCommand {
    from_entity: Entity,
    team: TeamIdx,
    from_position: Vec2,
    to_direction: Vec2,
}

#[derive(Component)]
pub struct BulletOwner {
    pub entity: Entity,
}

impl Command for SpawnBulletCommand {
    fn apply(self, world: &mut World) {
        world.spawn((
            Transform {
                translation: self.from_position.extend(2f32),
                ..default()
            },
            MoveSpeed(400f32),
            MoveDirection(self.to_direction),
            DespawnAfter {
                time_to_destroy: world.get_resource::<Time>().unwrap().elapsed_seconds() + 1f32,
            },
            BulletOwner {
                entity: self.from_entity,
            },
            self.team,
            RemoveOnRespawn,
        ));
        world.send_event(EventBulletSpawn {
            origin: self.from_position,
        });
    }
}

pub trait CommandsSpawnBullet {
    fn spawn_bullet(
        &mut self,
        from_entity: Entity,
        from_position: Vec2,
        direction: Vec2,
        team: TeamIdx,
        cooldown: &Cooldown,
        time: &Res<Time>,
    ) -> Result<&mut Self, ()>;
}

impl CommandsSpawnBullet for Commands<'_, '_> {
    fn spawn_bullet(
        &mut self,
        from_entity: Entity,
        from_position: Vec2,
        direction: Vec2,
        team: TeamIdx,
        cooldown: &Cooldown,
        time: &Res<Time>,
    ) -> Result<&mut Self, ()> {
        if direction == Vec2::ZERO {
            return Err(());
        }
        if time.elapsed_seconds() < cooldown.start_time + cooldown.duration {
            return Err(());
        }

        self.add(SpawnBulletCommand {
            from_entity,
            from_position,
            to_direction: direction,
            team,
        });
        Ok(self)
    }
}

pub fn bullet_sounds(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut ev_bullets: EventReader<EventBulletSpawn>,
) {
    for e in ev_bullets.iter() {
        commands.spawn((SpatialAudioBundle {
            source: asset_server.load("sounds/pew1.ogg"),
            settings: PlaybackSettings::ONCE,
            spatial: SpatialSettings::new(Transform::IDENTITY, 10f32, Vec3::ZERO),
        },));
    }
}
