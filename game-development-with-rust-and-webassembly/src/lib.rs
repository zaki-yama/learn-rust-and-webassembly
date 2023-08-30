#[macro_use]
mod browser;
mod engine;
mod game;

use anyhow::Result;
use engine::GameLoop;
use game::WalkTheDog;



use serde::Deserialize;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;


#[derive(Deserialize)]
struct Sheet {
    frames: HashMap<String, Cell>,
}

#[derive(Deserialize)]
struct Rect {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

#[derive(Deserialize)]
struct Cell {
    frame: Rect,
}

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let _context = browser::context().expect("Could not get browser context");

    browser::spawn_local(async move {
        let game = WalkTheDog::new();

        GameLoop::start(game)
            .await
            .expect("Could not start game loop");
    });
    Ok(())
}
