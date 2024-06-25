use console_log::*;
use leptos::*;
use leptos_app::components_leptos::first_comp::*;
use leptos_router::*;
fn main() {
    // mount_to_body(|| view! {<p>"Hello World"</p>})
    _ = init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <Router>
            <Routes>
            <Route path="/" view=Counter/>
            <Route path="/about" view=About/>
            </Routes>
        </Router>}
    });
}
