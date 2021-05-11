mod util;
mod life;

use util::{Point, util::println};
use wasm_bindgen::prelude::*;

// If you don't want to use `wee_alloc`, you can safely delete this.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    println("Hello, my rusty world!");
    println("this is a point");

    let p1 = Point::new(0.,0.);
    let p2 = p1.added(&Point::new(3.,4.));
    let p3 = p2.added(&Point::new(3.,100.));

    println(format!("point 1: {:?}", p1).as_str()); 
    println(format!("point 2: {:?}", p2).as_str()); 
    println(format!("point 3: {:?}", p3).as_str()); 

    // alert("Hello, henkiepenkie!");

    Ok(())
}