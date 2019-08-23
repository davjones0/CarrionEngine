extern crate specs;
use specs::prelude::*;
use specs::{Builder, DispatcherBuilder, World, Dispatcher};
use engine::components::{Position, Movement, Icon, IsPlayer};
use engine::systems::{ProcessMovement};

pub struct Game_World {
    pub world: World,
    pub dispatcher: Dispatcher<'static, 'static>
}

impl Game_World {
    pub fn new() -> Game_World {
        let _dispatch = DispatcherBuilder::new()
            .with(ProcessMovement, "process_movement", &[])
            .build();
        
        let mut build_world = World::new();
        build_world.register::<Position>();
        build_world.register::<Movement>();
        build_world.register::<Icon>();
        build_world.register::<IsPlayer>();
        
        Game_World {
            world: build_world,
            dispatcher: _dispatch
        }
    }

    pub fn dispatch(&mut self) {
        self.dispatcher.dispatch(&mut self.world);
        self.world.maintain();
    }

    pub fn create_player(&mut self) {
        self.world
            .create_entity()
            .with(Icon { value: String::from("@")})
            .with(Position::new())
            .with(Movement::new())
            .with(IsPlayer)
            .build();
            
        self.world.maintain();
    }
}