//! Frontend components using DaisyUI styling.
//!
//! DAISYUI REMOVAL: Replace DaisyUI classes with original class names.
//! Original classes are in comments next to each component. Also see
//! styles-vanilla.css for the original CSS.

pub mod math;

use sycamore::prelude::*;
use sycamore::web::on_mount;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = ["window", "electronAPI"], js_name = rustHello)]
    fn rust_hello() -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = window, js_name = initDemoChart)]
    fn init_demo_chart(canvas_id: &str);
}

/// Counter component demonstrating Sycamore reactivity.
///
/// DAISYUI classes used: card, card-body, btn, btn-primary, btn-secondary, badge, badge-lg
/// Original class: "counter"
#[component]
fn Counter() -> View {
    let count = create_signal(0i32);

    view! {
        // DAISYUI: card card-body -> Original: div(class="counter")
        div(class="card bg-base-200 shadow-xl") {
            div(class="card-body") {
                h2(class="card-title text-primary") { "Sycamore Counter" }
                p(class="flex items-center gap-2") {
                    "Count: "
                    // DAISYUI: badge badge-lg badge-accent
                    span(class="badge badge-lg badge-accent") { (count.get()) }
                }
                div(class="card-actions justify-start mt-2") {
                    // DAISYUI: btn btn-primary -> Original: button
                    button(class="btn btn-primary", on:click=move |_| count.set(math::add(count.get(), 1))) {
                        "Increment"
                    }
                    // DAISYUI: btn btn-secondary -> Original: button
                    button(class="btn btn-secondary", on:click=move |_| count.set(math::add(count.get(), -1))) {
                        "Decrement"
                    }
                }
            }
        }
    }
}

/// Backend demo component showing IPC with Neon backend.
///
/// DAISYUI classes used: card, card-body, alert, alert-info, btn, btn-accent
/// Original class: "backend-demo"
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
        // DAISYUI: card card-body -> Original: div(class="backend-demo")
        div(class="card bg-base-200 shadow-xl") {
            div(class="card-body") {
                h2(class="card-title text-primary") { "Neon Backend Demo" }
                // DAISYUI: alert alert-info -> Original: p
                div(class="alert alert-info") {
                    span { (message.get_clone()) }
                }
                div(class="card-actions justify-start mt-2") {
                    // DAISYUI: btn btn-accent -> Original: button
                    button(class="btn btn-accent", on:click=call_backend) {
                        "Call Rust Backend"
                    }
                }
            }
        }
    }
}

/// Chart demo component showing JS library integration.
///
/// DAISYUI classes used: card, card-body
/// Custom class: chart-container (height required for Chart.js)
/// Original class: "chart-demo"
#[component]
fn ChartDemo() -> View {
    on_mount(move || {
        init_demo_chart("demo-chart");
    });

    view! {
        // DAISYUI: card card-body -> Original: div(class="chart-demo")
        div(class="card bg-base-200 shadow-xl") {
            div(class="card-body") {
                h2(class="card-title text-primary") { "Chart.js Demo" }
                p(class="text-base-content/70") { "Example JavaScript library integration" }
                // Custom class: chart-container (needs fixed height for Chart.js)
                div(class="chart-container") {
                    canvas(id="demo-chart") {}
                }
            }
        }
    }
}

/// Main application component.
///
/// DAISYUI/Tailwind classes used: container, mx-auto, p-8, flex, flex-col, gap-6
/// Original: main element with no specific classes
#[component]
fn App() -> View {
    view! {
        // DAISYUI: container mx-auto -> centers content with max-width
        main(class="container mx-auto p-8 max-w-4xl") {
            // DAISYUI: text-center for header alignment
            div(class="text-center mb-8") {
                h1(class="text-4xl font-bold") { "{{project-name}}" }
                p(class="subtitle mt-2") {
                    "{{description}}"
                }
            }
            // DAISYUI: flex flex-col gap-6 -> vertical stack with spacing
            div(class="flex flex-col gap-6") {
                Counter {}
                BackendDemo {}
                ChartDemo {}
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    console_error_panic_hook::set_once();
    sycamore::render(App);
}
