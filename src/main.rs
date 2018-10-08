extern crate yew;
extern crate super_wasm;
use yew::prelude::*;

fn main() {
    yew::initialize();
    App::<super_wasm::Model>::new().mount_to_body();
    yew::run_loop();
}
