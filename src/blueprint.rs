use crate::prelude::*;

pub trait BlueprintAppExt {
    fn register_entity_blueprint<C>(&mut self) -> &mut Self
    where
        C: Component + Default + EntityBlueprint,
        EntityBlueprintBundle<C>: LdtkEntity + Bundle;

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

pub trait EntityBlueprint {
    const NAME: &'static str;

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

pub trait TileBlueprint {
    const NAME: &'static str;
    const ID: i32;

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

    fn components() -> impl Bundle;
}

#[derive(Bundle, Default, LdtkIntCell)]
pub struct TileBlueprintBundle<C>
where
    C: Component + Default,
{
    component: C,
}
