pub mod math;

use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = ["window", "electronAPI"], js_name = rustHello)]
    fn rust_hello() -> js_sys::Promise;
}

#[component]
fn Counter() -> View {
    let count = create_signal(0i32);

    view! {
        div(class="counter") {
            h2 { "Sycamore Counter" }
            p { "Count: " (count.get()) }
            button(on:click=move |_| count.set(math::add(count.get(), 1))) {
                "Increment"
            }
            button(on:click=move |_| count.set(math::add(count.get(), -1))) {
                "Decrement"
            }
        }
    }
}

#[component]
fn BackendDemo() -> View {
    let message = create_signal(String::from("Click to call Rust backend..."));

    let call_backend = move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            match wasm_bindgen_futures::JsFuture::from(rust_hello()).await {
                Ok(result) => {
                    if let Some(s) = result.as_string() {
                        message.set(s);
                    }
                }
                Err(e) => {
                    log(&format!("Error calling backend: {:?}", e));
                    message.set(String::from("Error calling backend"));
                }
            }
        });
    };

    view! {
        div(class="backend-demo") {
            h2 { "Neon Backend Demo" }
            p { (message.get_clone()) }
            button(on:click=call_backend) {
                "Call Rust Backend"
            }
        }
    }
}

#[component]
fn App() -> View {
    view! {
        main {
            h1 { "{{project-name}}" }
            p(class="subtitle") {
                "{{description}}"
            }
            Counter {}
            hr {}
            BackendDemo {}
        }
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    console_error_panic_hook::set_once();
    sycamore::render(App);
}
