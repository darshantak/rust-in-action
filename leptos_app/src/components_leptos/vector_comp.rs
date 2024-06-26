
use leptos::*;

#[component]
pub fn Counters() -> impl IntoView{
    let length = 5;
    let counters = (1..=length).map(|n| create_signal(n));
    let counter_buttons = counters.map(|(count, set_count)|  {
        view! {
            <li>
                <button on:click=move |_| {set_count.update(|n| *n+=1)}>
                    {count}
                </button>
            </li>
        }}
    ).collect_view();
    view! {
        <ul>{counter_buttons}</ul>
}
}