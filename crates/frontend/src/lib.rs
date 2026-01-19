//! Frontend components using DaisyUI styling.
//!
//! DAISYUI REMOVAL: Replace DaisyUI classes with original class names.
//! Original classes are in comments next to each component. Also see
//! styles-vanilla.css for the original CSS.

pub mod config;
pub mod math;
pub mod state;

use config::AppConfig;
use state::AppState;
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

    #[wasm_bindgen(js_namespace = ["window", "electronAPI"], js_name = loadConfig)]
    fn load_config() -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = ["window", "electronAPI"], js_name = saveConfig)]
    fn save_config(config_json: &str) -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = ["window", "electronAPI"], js_name = getConfigPath)]
    fn get_config_path() -> js_sys::Promise;
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
// Settings & State Components
// ============================================================================

/// Settings card component for config interaction.
/// Allows editing username, dark mode, notifications, and saving to disk.
#[component]
fn SettingsCard() -> View {
    let app_state = use_context::<AppState>();

    let username = create_signal(String::new());
    let dark_mode = create_signal(false);
    let notifications = create_signal(true);
    let config_path = create_signal(String::from("Loading..."));
    let save_status = create_signal(String::new());

    // Load config on mount
    on_mount(move || {
        wasm_bindgen_futures::spawn_local(async move {
            // Get config path
            match wasm_bindgen_futures::JsFuture::from(get_config_path()).await {
                Ok(result) => {
                    if let Some(s) = result.as_string() {
                        config_path.set(s);
                    }
                }
                Err(_) => config_path.set("Error getting path".to_string()),
            }

            // Load config
            match wasm_bindgen_futures::JsFuture::from(load_config()).await {
                Ok(result) => {
                    if let Some(json) = result.as_string() {
                        if let Ok(config) = serde_json::from_str::<AppConfig>(&json) {
                            username.set(config.username.clone());
                            dark_mode.set(config.dark_mode);
                            notifications.set(config.notifications_enabled);
                            app_state.username.set(config.username);
                        }
                    }
                }
                Err(e) => log(&format!("Error loading config: {:?}", e)),
            }
        });
    });

    let on_save = move |_| {
        let config = AppConfig {
            username: username.get_clone(),
            dark_mode: dark_mode.get(),
            notifications_enabled: notifications.get(),
            auto_save_interval: 300,
            theme: "system".to_string(),
        };

        // Update global state
        app_state.username.set(config.username.clone());

        let json = serde_json::to_string(&config).unwrap_or_default();

        wasm_bindgen_futures::spawn_local(async move {
            match wasm_bindgen_futures::JsFuture::from(save_config(&json)).await {
                Ok(result) => {
                    if result.as_bool() == Some(true) {
                        save_status.set("Saved!".to_string());
                    } else {
                        save_status.set("Save failed".to_string());
                    }
                }
                Err(_) => save_status.set("Error saving".to_string()),
            }

            // Clear status after 2 seconds
            gloo_timers::callback::Timeout::new(2000, move || {
                save_status.set(String::new());
            })
            .forget();
        });
    };

    view! {
        div(class="card bg-base-200 shadow-xl") {
            div(class="card-body") {
                h2(class="card-title text-primary") { "Settings" }

                // Username input
                div(class="form-control") {
                    label(class="label") {
                        span(class="label-text") { "Username" }
                    }
                    input(
                        r#type="text",
                        class="input input-bordered",
                        placeholder="Enter username",
                        bind:value=username,
                    )
                }

                // Dark mode toggle
                div(class="form-control") {
                    label(class="label cursor-pointer") {
                        span(class="label-text") { "Dark Mode" }
                        input(
                            r#type="checkbox",
                            class="toggle toggle-primary",
                            bind:checked=dark_mode,
                        )
                    }
                }

                // Notifications checkbox
                div(class="form-control") {
                    label(class="label cursor-pointer") {
                        span(class="label-text") { "Enable Notifications" }
                        input(
                            r#type="checkbox",
                            class="checkbox checkbox-primary",
                            bind:checked=notifications,
                        )
                    }
                }

                // Save button
                div(class="card-actions justify-start mt-4") {
                    button(class="btn btn-primary", on:click=on_save) { "Save Settings" }
                    (if !save_status.get_clone().is_empty() {
                        view! {
                            span(class="badge badge-success ml-2") { (save_status.get_clone()) }
                        }
                    } else {
                        view! {}
                    })
                }

                // Config path
                div(class="text-xs text-base-content/50 mt-4") {
                    "Config: " (config_path.get_clone())
                }
            }
        }
    }
}

/// Global state card component showing state interaction.
/// Displays username from state, counter with +/- buttons, and busy toggle.
#[component]
fn GlobalStateCard() -> View {
    let app_state = use_context::<AppState>();

    view! {
        div(class="card bg-base-200 shadow-xl") {
            div(class="card-body") {
                h2(class="card-title text-primary") { "Global State" }

                // Username stat
                div(class="stat p-0") {
                    div(class="stat-title") { "Current User" }
                    div(class="stat-value text-lg") {
                        (if app_state.username.get_clone().is_empty() {
                            "(not set)".to_string()
                        } else {
                            app_state.username.get_clone()
                        })
                    }
                }

                // Counter
                div(class="flex items-center gap-4 mt-4") {
                    span(class="font-medium") { "Counter:" }
                    div(class="join") {
                        button(
                            class="btn btn-outline btn-sm join-item",
                            on:click=move |_| app_state.decrement(),
                        ) { "-" }
                        span(class="badge badge-secondary badge-lg join-item px-4") {
                            (app_state.counter.get())
                        }
                        button(
                            class="btn btn-outline btn-sm join-item",
                            on:click=move |_| app_state.increment(),
                        ) { "+" }
                    }
                }

                // Busy toggle
                div(class="form-control mt-4") {
                    label(class="label cursor-pointer") {
                        span(class="label-text") { "App Busy" }
                        input(
                            r#type="checkbox",
                            class="toggle toggle-warning",
                            bind:checked=app_state.is_busy,
                        )
                    }
                }

                // Loading spinner when busy
                (if app_state.is_busy.get() {
                    view! {
                        div(class="flex items-center gap-2 mt-2") {
                            span(class="loading loading-spinner loading-sm") {}
                            span(class="text-sm text-base-content/70") { "Working..." }
                        }
                    }
                } else {
                    view! {}
                })
            }
        }
    }
}

/// Quick actions card demonstrating cross-component state interaction.
/// Notification input, counter presets, and notification display.
#[component]
fn QuickActionsCard() -> View {
    let app_state = use_context::<AppState>();
    let notification_input = create_signal(String::new());

    let send_notification = move |_| {
        let msg = notification_input.get_clone();
        if !msg.is_empty() {
            app_state.notify(msg);
            notification_input.set(String::new());
        }
    };

    view! {
        div(class="card bg-base-200 shadow-xl") {
            div(class="card-body") {
                h2(class="card-title text-primary") { "Quick Actions" }

                // Notification input
                div(class="join w-full") {
                    input(
                        r#type="text",
                        class="input input-bordered join-item flex-1",
                        placeholder="Enter notification message",
                        bind:value=notification_input,
                    )
                    button(
                        class="btn btn-accent join-item",
                        on:click=send_notification,
                    ) { "Send" }
                }

                // Counter presets
                div(class="flex gap-2 mt-4") {
                    button(
                        class="btn btn-outline btn-sm",
                        on:click=move |_| app_state.set_counter(0),
                    ) { "Reset" }
                    button(
                        class="btn btn-outline btn-sm",
                        on:click=move |_| app_state.set_counter(10),
                    ) { "Set 10" }
                    button(
                        class="btn btn-outline btn-sm",
                        on:click=move |_| app_state.set_counter(100),
                    ) { "Set 100" }
                }

                // Active notification display
                (if let Some(notification) = app_state.notification.get_clone() {
                    view! {
                        div(class="alert alert-info mt-4") {
                            span { (notification) }
                            button(
                                class="btn btn-ghost btn-xs",
                                on:click=move |_| app_state.clear_notification(),
                            ) { "Dismiss" }
                        }
                    }
                } else {
                    view! {}
                })
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
                // State and Config cards in a grid
                div(class="grid grid-cols-1 lg:grid-cols-3 gap-4") {
                    SettingsCard {}
                    GlobalStateCard {}
                    QuickActionsCard {}
                }
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
    // Create and provide global app state
    let app_state = AppState::new();
    provide_context(app_state);

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
