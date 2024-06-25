use leptos::*;

#[component]
pub fn Counter() -> impl IntoView{
    let (count,set_count) = create_signal(0);
    view! {
        <div>
            <h1> "Counter": {count}</h1>
            <button on:click = move |_| {set_count.update(|n| *n+=1)} >"Increment"</button>
            <button on:click=move |_|{set_count.update(|n| *n-=1)}>"Decrement"</button>
        </div>
        
    }
}