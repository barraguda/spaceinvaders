use kinode_process_lib::{await_message, call_init, homepage, http, println, Address};
static SPACE_SVG: &str = include_str!("../../pkg/ui/space.svg");

wit_bindgen::generate!({
    path: "target/wit",
    world: "process-v1",
});

call_init!(init);
fn init(our: Address) {
    println!("--öIIIö--");
    let mut server = http::server::HttpServer::new(5);
    server.serve_ui(&our, "ui", vec!["/"], http::server::HttpBindingConfig::default()).unwrap();
    homepage::add_to_homepage("space invaders", Some(SPACE_SVG), Some("/"), None);
    loop {
        match await_message() {
            _ => {}
        };
    }
}
