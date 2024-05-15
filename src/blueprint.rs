use crate::prelude::*;

#[macro_export]
macro_rules! ldtk_entity {
    ($name:ident) => {
        #[derive(Component, Debug, Default)]
        pub struct $name;

        #[derive(Bundle, Default, LdtkEntity)]
        struct BlueprintBundle {
            marker: $name,
            #[sprite_sheet_bundle]
            sprite_sheet_bundle: SpriteSheetBundle,
        }
    };
}

#[macro_export]
macro_rules! ldtk_tile {
    ($name:ident) => {
        #[derive(Component, Debug, Default)]
        pub struct $name;

        #[derive(Bundle, Default, LdtkIntCell)]
        struct BlueprintBundle {
            marker: $name,
        }
    };
}

pub trait BlueprintAppExt {
    fn register_entity_blueprint<C: Component + EntityBlueprint<impl LdtkEntity + Bundle>>(
        &mut self,
    ) -> &mut Self;

    fn register_entity_blueprint_wrapper<C, B>(&mut self) -> &mut Self
    where
        C: Component + EntityBlueprint<B>,
        B: LdtkEntity + Bundle;

    fn register_tile_blueprint<C: Component + TileBlueprint<impl LdtkIntCell + Bundle>>(
        &mut self,
    ) -> &mut Self;

    fn register_tile_blueprint_wrapper<C, B>(&mut self) -> &mut Self
    where
        C: Component + TileBlueprint<B>,
        B: LdtkIntCell + Bundle;
}

impl BlueprintAppExt for App {
    fn register_entity_blueprint<C: Component + EntityBlueprint<impl LdtkEntity + Bundle>>(
        &mut self,
    ) -> &mut Self {
        self.register_entity_blueprint_wrapper::<C, _>()
    }

    fn register_entity_blueprint_wrapper<C, B>(&mut self) -> &mut Self
    where
        C: Component + EntityBlueprint<B>,
        B: LdtkEntity + Bundle,
    {
        self.register_ldtk_entity::<B>(C::NAME)
            .add_systems(Update, C::hydrate)
    }

    fn register_tile_blueprint<C: Component + TileBlueprint<impl LdtkIntCell + Bundle>>(
        &mut self,
    ) -> &mut Self {
        self.register_tile_blueprint_wrapper::<C, _>()
    }

    fn register_tile_blueprint_wrapper<C, B>(&mut self) -> &mut Self
    where
        C: Component + TileBlueprint<B>,
        B: LdtkIntCell + Bundle,
    {
        self.register_ldtk_int_cell::<B>(C::ID)
            .add_systems(Update, C::hydrate)
    }
}

pub trait EntityBlueprint<B>
where
    B: LdtkEntity + Bundle,
{
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

pub trait TileBlueprint<B>
where
    B: LdtkIntCell + Bundle,
{
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
