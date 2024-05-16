use crate::prelude::*;

pub trait BlueprintAppExt {
    /// Registers an entity blueprint for entity imports from LDtk scenes.
    fn register_entity_blueprint<C>(&mut self) -> &mut Self
    where
        C: Component + Default + EntityBlueprint,
        EntityBlueprintBundle<C>: LdtkEntity + Bundle;

    /// Registers a tile blueprint for int cell imports from LDtk scenes.
    fn register_tile_blueprint<C>(&mut self) -> &mut Self
    where
        C: Component + Default + TileBlueprint,
        TileBlueprintBundle<C>: LdtkIntCell + Bundle;
}

impl BlueprintAppExt for App {
    fn register_entity_blueprint<C>(&mut self) -> &mut Self
    where
        C: Component + Default + EntityBlueprint,
        EntityBlueprintBundle<C>: LdtkEntity + Bundle,
    {
        self.register_ldtk_entity::<EntityBlueprintBundle<C>>(C::NAME)
            .add_systems(Update, C::hydrate)
    }

    fn register_tile_blueprint<C>(&mut self) -> &mut Self
    where
        C: Component + Default + TileBlueprint,
        TileBlueprintBundle<C>: LdtkIntCell + Bundle,
    {
        self.register_ldtk_int_cell::<TileBlueprintBundle<C>>(C::ID)
            .add_systems(Update, C::hydrate)
    }
}

/// A trait for easily ingesting LDtk entities using the blueprint pattern.
pub trait EntityBlueprint {
    const NAME: &'static str;

    /// Hydrates newly added LDtk entities with any associated components.
    fn hydrate(mut commands: Commands, new_entities: Query<Entity, Added<Self>>)
    where
        Self: Component,
        Self: Sized,
    {
        for entity in new_entities.iter() {
            commands
                .entity(entity)
                .insert((Name::new(Self::NAME), Self::components()));
        }
    }

    /// Returns the components associated with this blueprint.
    fn components() -> impl Bundle;
}

#[derive(Bundle, Default, LdtkEntity)]
pub struct EntityBlueprintBundle<C>
where
    C: Component + Default,
{
    component: C,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

/// A trait for easily ingesting LDtk int cells using the blueprint pattern.
pub trait TileBlueprint {
    const NAME: &'static str;
    const ID: i32;

    /// Hydrates newly added LDtk int cells with any associated components.
    fn hydrate(mut commands: Commands, new_entities: Query<Entity, Added<Self>>)
    where
        Self: Component,
        Self: Sized,
    {
        for entity in new_entities.iter() {
            commands
                .entity(entity)
                .insert((Name::new(Self::NAME), Self::components()));
        }
    }

    /// Returns the components associated with this blueprint.
    fn components() -> impl Bundle;
}

#[derive(Bundle, Default, LdtkIntCell)]
pub struct TileBlueprintBundle<C>
where
    C: Component + Default,
{
    component: C,
}
