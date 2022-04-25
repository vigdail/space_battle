use bevy::{
    asset::Asset,
    ecs::system::{Command, EntityCommands},
    prelude::*,
};

pub trait FromRaw {
    type Raw;
    fn from_raw(raw: &Self::Raw, world: &mut World) -> Self;
}

pub trait Prefab: Send + Sync + 'static {
    fn spawn(&self, world: &mut World) {
        let entity = world.spawn().id();
        self.apply(entity, world);
    }

    fn apply(&self, entity: Entity, world: &mut World);
}

pub struct SpawnPrefab<T: Prefab> {
    prefab: T,
}

impl<T> Command for SpawnPrefab<T>
where
    T: Prefab,
{
    fn write(self, world: &mut World) {
        self.prefab.spawn(world);
    }
}

pub struct ApplyPrefab<T: Prefab> {
    entity: Entity,
    prefab: T,
}

impl<T> Command for ApplyPrefab<T>
where
    T: Prefab,
{
    fn write(self, world: &mut World) {
        self.prefab.apply(self.entity, world);
    }
}

pub trait EntityPrefabCommands {
    fn apply_prefab<T: Prefab>(&mut self, prefab: T) -> &mut Self;
}

impl EntityPrefabCommands for EntityCommands<'_, '_, '_> {
    fn apply_prefab<T: Prefab>(&mut self, prefab: T) -> &mut Self {
        let entity = self.id();
        self.commands().add(ApplyPrefab { entity, prefab });
        self
    }
}

pub trait RegisterPrefab {
    fn register_prefab<T>(&mut self) -> &mut Self
    where
        T: Prefab + Asset + Clone;
}

impl RegisterPrefab for App {
    fn register_prefab<T>(&mut self) -> &mut Self
    where
        T: Prefab + Asset + Clone,
    {
        self.add_asset::<T>().add_system(apply_prefab_handle::<T>)
    }
}

fn apply_prefab_handle<T>(
    mut commands: Commands,
    assets: Res<Assets<T>>,
    query: Query<(Entity, &Handle<T>)>,
) where
    T: Prefab + Asset + Clone,
{
    for (entity, handle) in query.iter() {
        if let Some(prefab) = assets.get(handle) {
            commands
                .entity(entity)
                .apply_prefab(prefab.clone())
                .remove::<Handle<T>>();
        }
    }
}
