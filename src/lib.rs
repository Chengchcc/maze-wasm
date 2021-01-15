#![allow(non_snake_case)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate cfg_if;

extern crate array_tool;


mod maze;
// mod stopwatch;

use wasm_bindgen::prelude::*;
// use web_sys::console;
use maze::Maze;

use std::sync::Mutex;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}


cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

lazy_static! {
    static ref MAZE: Mutex<Maze> = Mutex::new(Maze::default()) ;
}


// // This is like the `main` function, except for JavaScript.
// #[wasm_bindgen(start)]
// pub fn main_js() -> Result<(), JsValue> {
//     // This provides better error messages in debug mode.
//     // It's disabled in release mode so it doesn't bloat up the file size.
//     #[cfg(debug_assertions)]
//     console_error_panic_hook::set_once();


//     // Your code goes here!
//     console::log_1(&JsValue::from_str("Hello world!"));

//     Ok(())
// }


#[wasm_bindgen]
pub fn init_maze(col: u64, row: u64) -> Result<(), JsValue> {
    set_panic_hook();

    let mut mazz = MAZE.lock().unwrap();
    mazz.reset(col, row);
    Ok(())
}

#[wasm_bindgen]
pub fn gen_maze() -> Result<JsValue, JsValue> {
    set_panic_hook();

    let mut mazz = MAZE.lock().unwrap();
    if mazz.len() == 0 {
        Err(JsValue::from_serde("maze not init").unwrap())
    }else {
        Ok(JsValue::from_serde(mazz.generate()).unwrap())
    }
}


#[wasm_bindgen]
pub fn cal_path() -> Result<JsValue, JsValue> {
    set_panic_hook();
      let mut mazz = MAZE.lock().unwrap();
    if mazz.len() == 0 {
        Err(JsValue::from_serde("maze not init").unwrap())
    }else {
        Ok(JsValue::from_serde(&mazz.cal_path()).unwrap())
    }
}