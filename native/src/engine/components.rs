use specs::{Component, VecStorage, NullStorage};

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

impl Position {
    pub fn new() -> Position {
        Position {
            x: 0,
            y: 0
        }
    }
}


#[derive(Debug)]
pub struct Movement {
    pub x: i32,
    pub y: i32,
}

impl Component for Movement {
    type Storage = VecStorage<Self>;
}

impl Movement {
    pub fn new() -> Movement {
        Movement {
            x: 0,
            y: 0
        }
    }
}


#[derive(Debug)]
pub struct Icon {
    pub value: String
}

impl Component for Icon {
    type Storage = VecStorage<Self>;
}

impl Icon {
    pub fn new() -> Icon {
        Icon {
            value: String::from(" ")
        }
    }
}


#[derive(Debug, Default)]
pub struct IsPlayer;

impl Component for IsPlayer {
    type Storage = NullStorage<Self>;
}