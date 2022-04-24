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

pub struct ApplyPrefabHandle<T: Prefab + Asset> {
    entity: Entity,
    handle: Handle<T>,
}

impl<T> Command for ApplyPrefabHandle<T>
where
    T: Prefab + Asset + Clone,
{
    fn write(self, world: &mut World) {
        let prefab = world
            .get_resource::<Assets<T>>()
            .and_then(|assets| assets.get(self.handle))
            .cloned()
            .unwrap(); // TODO: Retry to apply prefab in some system

        prefab.apply(self.entity, world);
    }
}

pub trait EntityPrefabCommands {
    fn apply_prefab<T: Prefab>(&mut self, prefab: T) -> &mut Self;
    fn apply_prefab_handle<T: Prefab + Asset + Clone>(&mut self, handle: Handle<T>) -> &mut Self;
}

impl EntityPrefabCommands for EntityCommands<'_, '_, '_> {
    fn apply_prefab<T: Prefab>(&mut self, prefab: T) -> &mut Self {
        let entity = self.id();
        self.commands().add(ApplyPrefab { entity, prefab });
        self
    }

    fn apply_prefab_handle<T: Prefab + Asset + Clone>(&mut self, handle: Handle<T>) -> &mut Self {
        let entity = self.id();
        self.commands().add(ApplyPrefabHandle { entity, handle });
        self
    }
}
