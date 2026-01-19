//! Global application state using Sycamore signals.
//!
//! Use `provide_context` in the root component to make AppState available
//! throughout the component tree. Access it in child components with `use_context`.

use sycamore::prelude::*;

/// Global application state accessible throughout the component tree.
#[derive(Clone, Copy)]
pub struct AppState {
    pub counter: Signal<i32>,
    pub is_busy: Signal<bool>,
    pub username: Signal<String>,
    pub notification: Signal<Option<String>>,
}

impl AppState {
    /// Creates a new AppState with default values.
    pub fn new() -> Self {
        Self {
            counter: create_signal(0),
            is_busy: create_signal(false),
            username: create_signal(String::new()),
            notification: create_signal(None),
        }
    }

    /// Increments the counter by 1.
    pub fn increment(&self) {
        self.counter.set(self.counter.get() + 1);
    }

    /// Decrements the counter by 1.
    pub fn decrement(&self) {
        self.counter.set(self.counter.get() - 1);
    }

    /// Sets the counter to a specific value.
    pub fn set_counter(&self, value: i32) {
        self.counter.set(value);
    }

    /// Shows a notification message.
    pub fn notify(&self, msg: impl Into<String>) {
        self.notification.set(Some(msg.into()));
    }

    /// Clears the current notification.
    pub fn clear_notification(&self) {
        self.notification.set(None);
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
