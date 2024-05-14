pub enum LocalEntity {
    Ball,
    Paddle,
}

impl<'a> From<LocalEntity> for &'a str {
    fn from(value: LocalEntity) -> Self {
        match value {
            LocalEntity::Ball => "Ball",
            LocalEntity::Paddle => "Paddle",
        }
    }
}

pub enum LocalTile {
    Wall,
}

impl From<LocalTile> for i32 {
    fn from(value: LocalTile) -> Self {
        match value {
            LocalTile::Wall => 1,
        }
    }
}
