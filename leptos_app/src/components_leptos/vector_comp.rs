use leptos::*;

#[component]
pub fn Counters() -> impl IntoView {
    let length = 5;
    let counters = (1..=length).map(|n| create_signal(n));
    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! {
                <li>
                    <button on:click=move |_| {set_count.update(|n| *n+=1)}>
                        {count}
                    </button>
                </li>
            }
        })
        .collect_view();
    view! {
            <ul>{counter_buttons}</ul>
    }
}

pub fn DynamicList() -> impl IntoView {
    let initial_length = 5;
    let mut next_counter_id = 0;
    let initial_counters = (0..initial_length)
        .map(|id| (id, create_signal(id + 1)))
        .collect::<Vec<_>>();
    let (counters, set_counters) = create_signal(initial_counters);

    let add_counter = move |_| {
        let sig = create_signal(next_counter_id+1);
        set_counters.update(move |counters| {
            counters.push((next_counter_id,sig))
        });
        next_counter_id+=1;
    };

    view! {

    }
}
