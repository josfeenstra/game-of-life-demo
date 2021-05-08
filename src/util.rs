pub mod util {

    use web_sys::console;
    use wasm_bindgen::prelude::*;
    use std::str;    

    pub fn println(message: &str) {

        let str = JsValue::from_str(message);
        console::log_1(&str);
    }
}


#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {

    pub fn new(x: f32, y: f32) -> Point {
        return Point {x, y};
    }

    pub fn add(&mut self, other: &Point) -> &Point {
        self.x += other.x;
        self.y += other.y;
        return self;
    }

    pub fn added(&self, other: &Point) -> Point {
        return Self::new(self.x + other.x, self.y + other.y);
    }
}

