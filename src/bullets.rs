use bevy::{ecs::system::Command, math::Vec3Swizzles, prelude::*};
use bevy_asset_loader::prelude::*;

use crate::{
    despawn_after::DespawnAfter,
    movement::{MoveDirection, MoveSpeed},
    player::Player,
    Cooldown, RemoveOnRespawn, TeamIdx,
};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<BulletAssets>();
        app.add_systems(PostUpdate, bullet_sounds);
    }
}

#[derive(AssetCollection, Resource)]
struct BulletAssets {
    #[asset(path = "sounds/pew1.ogg")]
    pew1: Handle<AudioSource>,
}

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

fn bullet_sounds(
    bullet_assets: Res<BulletAssets>,
    mut commands: Commands,
    mut ev_bullets: EventReader<EventBulletSpawn>,
    listener: Query<(&Transform), With<Player>>,
) {
    for e in ev_bullets.iter() {
        let listener = listener.single();
        commands.spawn((SpatialAudioBundle {
            source: bullet_assets.pew1.clone(),
            settings: PlaybackSettings::ONCE,
            spatial: SpatialSettings::new(
                Transform::IDENTITY,
                5f32,
                (e.origin - listener.translation.xy())
                    .normalize_or_zero()
                    .extend(0f32),
            ),
        },));
    }
}
