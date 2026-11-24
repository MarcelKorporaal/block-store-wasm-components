pub mod bindings {
    wit_bindgen::generate!({ generate_all });

    use crate::HelloWorld;
    export!(HelloWorld);
}

use crate::bindings::exports::betty_blocks::hello_world::hello_world::Guest;

pub struct HelloWorld;

impl Guest for HelloWorld {
    fn hello_world() -> String {
        String::from("Hello world!")
    }
}

#[test]
fn returns_hello_world() {
    assert_eq!(HelloWorld::hello_world(), "Hello world!");
}
