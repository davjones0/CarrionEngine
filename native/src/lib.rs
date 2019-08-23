#[macro_use]
extern crate neon;
extern crate specs;
extern crate toml;
extern crate serde;
mod engine;
mod settings;

use neon::prelude::*;
use specs::{World, Builder};
use engine::ecs;
use settings::controls::Config;

static mut Game: Option<ecs::Game_World> = None; 

fn init_game(mut cx: FunctionContext) -> JsResult<JsBoolean>{
    unsafe {
        Game = Some(ecs::Game_World::new());
    }
    Ok(cx.boolean(true))
}

fn dispatch(mut cx: FunctionContext) -> JsResult<JsBoolean> {
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
    Ok(cx.boolean(true))
}

fn handle_key_press(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    unsafe {
        let _game: &mut ecs::Game_World;
        match Game {
            Some(ref mut x) => {
                _game = &mut *x;
            },
            None => panic!(),
        }
        let key = cx.argument::<JsString>(0)?.value();
        // match key {
        //     "ArrowUp" => {

        //     },
        //     "ArrowDown" => {

        //     },
        //     "ArrowLeft" => {

        //     },
        //     "ArrowRight" => {

        //     }
        // }
    }

    Ok(cx.boolean(true))
}

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

register_module!(mut cx, {
    cx.export_function("hello", hello)?;
    cx.export_function("init_game", init_game)?;
    cx.export_function("dispatch", dispatch)?;
    cx.export_function("handleKeyPress", handle_key_press)?;
    Ok(())
});
