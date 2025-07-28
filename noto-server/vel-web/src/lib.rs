use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    let greet = &format!("Hello, {}!", name);
    alert(greet);
    String::from(greet)
}

#[wasm_bindgen]
pub fn add(a: isize, b: isize) -> isize {
    a + b
}
