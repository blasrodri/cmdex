#[macro_use]
extern crate clap;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate fuzzy_matcher;

#[macro_use]
pub mod commands;
pub mod cli;
pub mod examples;
pub mod query;
pub mod utils;

extern crate cfg_if;
extern crate wasm_bindgen;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn greet() {
    println!("{}", "Hello, wasm-game-of-life!");
}
