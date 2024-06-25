use leptos::*;
use svg::view;
use web_sys::console::count;

#[component]
pub fn Counter() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count.get() * 2;
    view! {
        <div>
            <h1> "Counter": {count}</h1>
            <button on:click = move |_| {set_count.update(|n| *n+=1)} class:red=move || count.get() % 2==1 >"Increment"</button>
            <button on:click=move |_|{set_count.update(|n| *n-=1)}>"Decrement"</button>
            <ProgressBar progress_value=move || count.get() />
            <ProgressBar progress_value=double_count />
            <ProgressBarWithIntoProp progress_value=count />
            <ProgressBarWithIntoProp progress_value=Signal::derive(double_count) />
        </div>

    }
}

#[component]
pub fn ProgressBar(
    progress_value: impl Fn() -> i32 + 'static,
    #[prop(default = 20)] max: u16,
) -> impl IntoView {
    view! {
        <progress max=max value=progress_value ></progress>
    }
}

#[component]
pub fn ProgressBarWithIntoProp(
    #[prop(default = 10)] max: i32,
    #[prop(into)] progress_value: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress max=max value=progress_value ></progress>
    }
}
#[component]
pub fn About() -> impl IntoView {
    view! {
        <h1> "This is the about page"</h1>
    }
}
