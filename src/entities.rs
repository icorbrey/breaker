pub enum LocalEntity {
    Ball,
    Paddle,
}

impl<'a> Into<&'a str> for LocalEntity {
    fn into(self) -> &'a str {
        match self {
            Self::Ball => "Ball",
            Self::Paddle => "Paddle",
        }
    }
}

pub enum LocalTile {
    Wall,
}

impl Into<i32> for LocalTile {
    fn into(self) -> i32 {
        match self {
            Self::Wall => 1,
        }
    }
}
