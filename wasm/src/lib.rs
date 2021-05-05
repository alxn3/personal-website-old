use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn add_two_ints(a: u32, b: u32) -> u32 {
   a + b
}
#[wasm_bindgen]
pub fn fib(n: u32) -> u32 {
   if n == 0 || n == 1 {
      return n;
   }

   fib(n - 1) + fib(n - 2)
}

#[wasm_bindgen]
pub fn HELLO() {
  log("HELLO!")
}
