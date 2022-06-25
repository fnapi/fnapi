#![feature(box_syntax)]
#![recursion_limit = "2048"]
#![allow(dead_code)]

#[macro_use]
extern crate napi_derive;

use std::{env, panic::set_hook};

use backtrace::Backtrace;

mod cli;
mod util;

#[napi::module_init]
fn init() {
    if cfg!(debug_assertions) || env::var("FNAPI_DEBUG").unwrap_or_default() == "1" {
        set_hook(Box::new(|panic_info| {
            let backtrace = Backtrace::new();
            println!("Panic: {:?}\nBacktrace: {:?}", panic_info, backtrace);
        }));
    }
}
