use wasm_bindgen::prelude::*;

#[no_mangle]
pub static data: &str = "foo baz";

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {} {}!", name, data));
}

const _: () = {
    #[link_section = "data_A"]
    static SECTION_CONTENT: [u8; 13] = *b"hello world A";
};