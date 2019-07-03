extern crate specs;
use specs::{Builder, DispatcherBuilder, World, Dispatcher};

pub struct Game_World {
    pub world: World,
    pub dispatcher: Dispatcher<'static, 'static>
}

impl Game_World {
    pub fn new() -> Game_World {
        let _dispatch = DispatcherBuilder::new()
            .build();
        
        Game_World {
            world: World::new(),
            dispatcher: _dispatch
        }
    }

    pub fn dispatch(&mut self) {
        self.dispatcher.dispatch(&mut self.world.res);
        self.world.maintain();
    }
}