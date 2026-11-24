mod bindings {
    wasmtime_testing_helper::bindgen!("main");

    wasmtime_testing_helper::setup!(Main);
}

#[test]
fn returns_hello_world() {
    let harness = bindings::harness();
    let mut component = bindings::instantiate(harness);

    let interface = component.component.betty_blocks_hello_world_hello_world();
    let greeting = interface.call_hello_world(&mut component.store).unwrap();

    assert_eq!(greeting, "Hello world!");
}
