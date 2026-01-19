//! Frontend components using DaisyUI styling.
//!
//! DAISYUI REMOVAL: Replace DaisyUI classes with original class names.
//! Original classes are in comments next to each component. Also see
//! styles-vanilla.css for the original CSS.

pub mod math;

use sycamore::prelude::*;
use sycamore::web::on_mount;
use wasm_bindgen::prelude::*;
use web_sys::{Element, PointerEvent};

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
            div(class="card-footer text-sm text-base-content/70") {
                "This counter is built with Sycamore's reactive signals."
            }
        }
    }
}

#[component]
fn Stuff() -> View {
    view! {
        div(class="flex w-full bg-base-200 shadow-xl") {
            Counter {}
            div(class="divider divider-horizontal") {}
            div(class="card card-xs") {
                div(class="card-body") {
                    h2(class="card-title text-primary") { "A DAISYUI Card" }
                    ul(class="list-disc list-inside mb-4 bg-base-100 rounded-box") {
                        li(class="list-row") { "Item One" }
                        li(class="list-row") { "Item Two" }
                        li(class="list-row") { "Item Three" }
                    }
                    p { "This is an example of a DAISYUI styled card component." }
                    button(class="btn btn-primary") { "A button" }
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

// ============================================================================
// Layout Components
// ============================================================================

/// Top navigation bar component.
/// Uses daisyUI navbar with start/center/end sections.
#[component]
fn TopBar() -> View {
    view! {
        div(class="navbar bg-base-200 shadow-lg") {
            div(class="navbar-start") {
                a(class="btn btn-ghost text-xl") { "{{project-name}}" }
            }
            div(class="navbar-center hidden lg:flex") {
                ul(class="menu menu-horizontal px-1") {
                    li { a { "Home" } }
                    li { a { "About" } }
                }
            }
            div(class="navbar-end") {
                button(class="btn btn-primary btn-sm") { "Action" }
            }
        }
    }
}

/// Left sidebar component with dynamic width.
#[component(inline_props)]
fn LeftSidebar(width: Signal<f64>) -> View {
    let style = move || format!("width: {}px", width.get());

    view! {
        aside(class="sidebar bg-base-200", style=style) {
            ul(class="menu p-4") {
                li(class="menu-title") { "Navigation" }
                li { a { "Dashboard" } }
                li { a { "Projects" } }
                li { a { "Settings" } }
            }
        }
    }
}

/// Right sidebar component with dynamic width.
#[component(inline_props)]
fn RightSidebar(width: Signal<f64>) -> View {
    let style = move || format!("width: {}px", width.get());

    view! {
        aside(class="sidebar bg-base-200", style=style) {
            div(class="p-4") {
                h3(class="font-bold mb-2") { "Details" }
                p(class="text-sm text-base-content/70") {
                    "Select an item to view details."
                }
            }
        }
    }
}

/// Resize handle component for dragging between panels.
#[component(inline_props)]
fn ResizeHandle(
    panel_width: Signal<f64>,
    resize_left: bool,
    min_width: f64,
    max_width: f64,
    is_any_dragging: Signal<bool>,
) -> View {
    let is_dragging = create_signal(false);
    let drag_start_x = create_signal(0.0);
    let start_width = create_signal(0.0);

    let handle_class = move || {
        if is_dragging.get() {
            "resize-handle dragging"
        } else {
            "resize-handle"
        }
    };

    let on_pointer_down = move |e: PointerEvent| {
        is_dragging.set(true);
        is_any_dragging.set(true);
        drag_start_x.set(e.client_x() as f64);
        start_width.set(panel_width.get());

        if let Some(target) = e.target() {
            if let Ok(element) = target.dyn_into::<Element>() {
                let _ = element.set_pointer_capture(e.pointer_id());
            }
        }
    };

    let on_pointer_move = move |e: PointerEvent| {
        if !is_dragging.get() {
            return;
        }

        let delta = e.client_x() as f64 - drag_start_x.get();
        let new_width = if resize_left {
            start_width.get() + delta
        } else {
            start_width.get() - delta
        };

        let clamped = new_width.max(min_width).min(max_width);
        panel_width.set(clamped);
    };

    let on_pointer_up = move |e: PointerEvent| {
        if !is_dragging.get() {
            return;
        }

        is_dragging.set(false);
        is_any_dragging.set(false);

        if let Some(target) = e.target() {
            if let Ok(element) = target.dyn_into::<Element>() {
                let _ = element.release_pointer_capture(e.pointer_id());
            }
        }
    };

    view! {
        div(
            class=handle_class,
            on:pointerdown=on_pointer_down,
            on:pointermove=on_pointer_move,
            on:pointerup=on_pointer_up,
        )
    }
}

/// Main content area wrapper.
#[component]
fn MainContent() -> View {
    view! {
        main(class="main-content p-6") {
            div(class="flex flex-col gap-6") {
                Stuff {}
                BackendDemo {}
                ChartDemo {}
            }
        }
    }
}

/// Main area component that orchestrates the resizable sidebar layout.
#[component]
fn MainArea() -> View {
    let left_width = create_signal(200.0);
    let right_width = create_signal(200.0);
    let is_dragging = create_signal(false);

    let area_class = move || {
        if is_dragging.get() {
            "main-area resizing"
        } else {
            "main-area"
        }
    };

    view! {
        div(class=area_class) {
            LeftSidebar(width=left_width)
            ResizeHandle(
                panel_width=left_width,
                resize_left=true,
                min_width=100.0,
                max_width=400.0,
                is_any_dragging=is_dragging,
            )
            MainContent {}
            ResizeHandle(
                panel_width=right_width,
                resize_left=false,
                min_width=100.0,
                max_width=400.0,
                is_any_dragging=is_dragging,
            )
            RightSidebar(width=right_width)
        }
    }
}

/// Application footer component.
#[component]
fn AppFooter() -> View {
    view! {
        footer(class="footer footer-center p-4 bg-base-200 text-base-content") {
            aside {
                p { "Built with Sycamore, Electron, and DaisyUI" }
            }
        }
    }
}

// ============================================================================
// Main Application
// ============================================================================

/// Main application component with standard layout.
///
/// Layout structure:
/// - TopBar: Navigation bar at top
/// - MainArea: Contains LeftSidebar, MainContent, RightSidebar with resize handles
/// - AppFooter: Footer at bottom
///
/// To remove sections, simply delete the corresponding component from this view.
#[component]
fn App() -> View {
    view! {
        div(class="app-layout") {
            TopBar {}
            MainArea {}
            AppFooter {}
        }
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    console_error_panic_hook::set_once();
    sycamore::render(App);
}
