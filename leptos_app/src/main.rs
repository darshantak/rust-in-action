use leptos::*;
use leptos_app::components_leptos::first_comp::Counter;
use console_log::*;
use log::*;
fn main() {
    // mount_to_body(|| view! {<p>"Hello World"</p>}) 
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| Counter());
    // Counter();
}