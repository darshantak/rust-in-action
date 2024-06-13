use leptos::*;
use leptos::ev::SubmitEvent;

// #[component]
// pub fn App() -> impl IntoView{
//     let (count,set_count) = create_signal(0);
//     view! {
//         <button 
//         on:click=move |_|{
//             set_count(3);
//         }>
//         "click me"
//         {move |_| count()}
//         </button>
//     }
// }
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button
            on:click=move |_| {
                // on stable, this is set_count.set(3);
                set_count.update(|n| *n+=1);
            }
        >
            "Click me: "
            // on stable, this is move || count.get();
            {move || count.get()}
        </button>
    }
}

fn main() {
    leptos::mount_to_body(|| view! {<App/>})
}