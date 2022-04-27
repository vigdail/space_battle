use bevy::{
    asset::Asset,
    ecs::system::{Command, EntityCommands},
    prelude::*,
    render::texture::DEFAULT_IMAGE_HANDLE,
};
use serde::{Deserialize, Serialize};

pub trait FromRaw {
    type Raw;
    fn from_raw(raw: &Self::Raw, world: &mut World) -> Self;
}

pub trait Prefab: Send + Sync + 'static {
    fn spawn(&self, world: &mut World) -> Entity {
        let entity = world.spawn().id();
        self.apply(entity, world);
        entity
    }

    fn apply(&self, entity: Entity, world: &mut World);
}

impl<T: Prefab> Prefab for Vec<T> {
    fn apply(&self, entity: Entity, world: &mut World) {
        let children = self
            .iter()
            .map(|prefab| prefab.spawn(world))
            .collect::<Vec<_>>();
        world.entity_mut(entity).push_children(&children);
    }
}

// impl<C: Component + Clone> Prefab for C {
//     fn apply(&self, entity: Entity, world: &mut World) {
//         world.entity_mut(entity).insert(self.clone());
//     }
// }

#[derive(Serialize, Deserialize, Clone)]
pub enum PrefabHandle<T> {
    Prefab(T),
    Asset(String),
}

impl<T: Clone + Asset> PrefabHandle<T> {
    pub fn as_handle(&self, world: &mut World) -> Handle<T> {
        match self {
            PrefabHandle::Prefab(prefab) => world.resource_mut::<Assets<T>>().add(prefab.clone()),
            PrefabHandle::Asset(path) => world.resource::<AssetServer>().get_handle(path),
        }
    }
}

impl<T: Asset + Prefab> Prefab for PrefabHandle<T> {
    fn apply(&self, entity: Entity, world: &mut World) {
        match self {
            PrefabHandle::Prefab(prefab) => prefab.apply(entity, world),
            PrefabHandle::Asset(path) => {
                let handle: Handle<T> = world.resource::<AssetServer>().get_handle(path);
                world.entity_mut(entity).insert(handle);
            }
        }
    }
}

impl<T: Prefab> From<T> for PrefabHandle<T> {
    fn from(prefab: T) -> Self {
        Self::Prefab(prefab)
    }
}

impl<T> From<String> for PrefabHandle<T> {
    fn from(path: String) -> Self {
        Self::Asset(path)
    }
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

#[macro_export]
macro_rules! prefab_loader {
    ($loader:ident, $prefab:ident, [$($exts:expr), +]) => {
        impl bevy::asset::AssetLoader for $loader {
            fn load<'a>(
                &'a self,
                bytes: &'a [u8],
                load_context: &'a mut bevy::asset::LoadContext,
            ) -> bevy::asset::BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
                Box::pin(async move {
                    let custom_asset = ron::de::from_bytes::<$prefab>(bytes)?;
                    load_context.set_default_asset(bevy::asset::LoadedAsset::new(custom_asset));
                    Ok(())
                })
            }

            fn extensions(&self) -> &[&str] {
                &[$($exts),+]
            }
        }
    };
}

#[derive(Bundle)]
pub struct SpriteBundle {
    pub sprite: Sprite,
    pub texture: Handle<Image>,
    pub visibility: Visibility,
}

impl Default for SpriteBundle {
    fn default() -> Self {
        Self {
            sprite: Default::default(),
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            visibility: Default::default(),
        }
    }
}
