use gloo_net::http::Request;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{console, window};
use yew::prelude::*;

#[function_component]
fn StatefulComponent() -> Html {
    let world_name = use_state(|| "???");
    let text = format!("Hello {} world!", *world_name);

    let on_click_button = move |_: MouseEvent| {
        world_name.set("yew");
    };
    let on_click_greet = move |_: MouseEvent| {
        let window = window().unwrap();
        window.alert_with_message("Hi!").unwrap();

        console::log_1(&JsValue::from_str("chen"));

        spawn_local(async move {
            let text = Request::get("https://example.com")
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            console::log_1(&JsValue::from_str(&text));
        });
    };
    html! {
        <div>
            <p>{ text }</p>
            <button onclick={on_click_button}>{ "Create world" }</button>
            <button onclick={on_click_greet}>{ "Greet" }</button>
        </div>
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <main>
            <StatefulComponent />
        </main>
    }
}
