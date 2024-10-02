// src/app.rs
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos::spawn::spawn_local;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct CounterArgs {
    count: i32,
}

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let increase_me = move |ev: MouseEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let count = count.get_untracked();
            let args = to_value(&CounterArgs { count }).unwrap();
            let new_value = invoke("increase", args).await.as_f64().unwrap();
            set_count.set(new_value as i32);
        });
    };
    let decrease_me = move |ev: MouseEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let count = count.get_untracked();
            let args = to_value(&CounterArgs { count }).unwrap();
            let new_value = invoke("decrease", args).await.as_f64().unwrap();
            set_count.set(new_value as i32);
        });
    };
    view! {
        <div>
            <button on:click=increase_me>"+1"</button>
            <button on:click=decrease_me>"-1"</button>
            <p class:red=move || count() < 0>"Counter: "{count}</p>
        </div>
    }
}
