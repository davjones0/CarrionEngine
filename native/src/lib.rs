#[macro_use]
extern crate neon;
extern crate specs;
mod engine;

use neon::prelude::*;
use specs::{World, Builder};
use engine::ecs;

static mut Game: Option<ecs::Game_World> = None; 

fn init_game() {
    unsafe {
        Game = Some(ecs::Game_World::new());
    }
}

fn dispatch() {
    unsafe {
        let _game: &mut ecs::Game_World;
        match Game {
            Some(ref mut x) => {
                _game = &mut *x;
            },
            None => panic!(),
        }

        _game.dispatch();
    }
}

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

register_module!(mut cx, {
    cx.export_function("hello", hello)
});
