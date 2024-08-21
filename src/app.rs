// src/app.rs
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    // added
    view! {
        <div>
            <button on:click=move |_| {
                set_count.update(|n| *n += 1);
            }>"+1"</button>
            <button on:click=move |_| {
                set_count.update(|n| *n -= 1);
            }>"-1"</button>
            <p class:red=move || count() < 0>"Counter: "{count}</p>
        </div>
    }
}
