use kinode_process_lib::{await_message, call_init, homepage, http, println, Address};
static SPACE_SVG: &str = include_str!("../../pkg/ui/space.svg");

wit_bindgen::generate!({
    path: "target/wit",
    world: "process-v0",
});

call_init!(init);
fn init(our: Address) {
    println!("--öIIIö--");

    http::serve_index_html(&our, "ui", false, false, vec!["/"]).unwrap();
    homepage::add_to_homepage("space invaders", Some(SPACE_SVG), Some("/"), None);
    loop {
        match await_message() {
            _ => {}
        };
    }
}
