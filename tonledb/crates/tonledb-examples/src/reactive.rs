//! Event-driven and reactive examples using druid GUI app
//!
//! This module demonstrates how to create a GUI application in druid where
//! a single event loop dispatches user input and redraw events.

use druid::widget::{Button, Flex, Label, TextBox};
use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc};
use std::time::{Duration, Instant};

/// Application state
#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub counter: i32,
    pub message: String,
    pub last_updated: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            counter: 0,
            message: "Welcome to the Druid App!".to_string(),
            last_updated: "Never".to_string(),
        }
    }
}

/// Update the timestamp in the app state
fn update_timestamp(data: &mut AppState) {
    let now = Instant::now();
    data.last_updated = format!("{:?}", now);
}

/// Create the main application widget
fn ui_builder() -> impl Widget<AppState> {
    // Create widgets
    let label = Label::new(|data: &AppState, _env: &druid::Env| {
        format!("Counter: {}", data.counter)
    });
    
    let message_label = Label::new(|data: &AppState, _env: &druid::Env| {
        data.message.clone()
    });
    
    let message_input = TextBox::new()
        .with_placeholder("Enter a message")
        .lens(AppState::message);
    
    let increment_button = Button::new("Increment")
        .on_click(|_ctx, data: &mut AppState, _env| {
            data.counter += 1;
            update_timestamp(data);
        });
    
    let decrement_button = Button::new("Decrement")
        .on_click(|_ctx, data: &mut AppState, _env| {
            data.counter -= 1;
            update_timestamp(data);
        });
    
    let reset_button = Button::new("Reset")
        .on_click(|_ctx, data: &mut AppState, _env| {
            data.counter = 0;
            update_timestamp(data);
        });
    
    // Layout the widgets
    Flex::column()
        .with_child(label.padding(5.0))
        .with_child(message_label.padding(5.0))
        .with_child(message_input.padding(5.0))
        .with_child(
            Flex::row()
                .with_child(increment_button.padding(5.0))
                .with_child(decrement_button.padding(5.0))
                .with_child(reset_button.padding(5.0))
        )
        .with_child(
            Label::new(|data: &AppState, _env: &druid::Env| {
                format!("Last updated: {}", data.last_updated)
            }).padding(5.0)
        )
        .center()
}

/// Run a simple druid GUI application
pub fn run_simple_gui_app() {
    println!("Starting simple Druid GUI application...");
    println!("The application window will appear shortly.");
    println!("Close the window to return to this console.");
    
    // Create the main window
    let main_window = WindowDesc::new(ui_builder())
        .title("TonleDB Examples - Druid App")
        .window_size((400.0, 300.0));
    
    // Create initial app state
    let initial_state = AppState::new();
    
    // Launch the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

/// A more complex reactive application with timers
#[derive(Clone, Data, Lens)]
pub struct TimerAppState {
    pub elapsed: f64,
    pub running: bool,
    pub message: String,
}

impl TimerAppState {
    pub fn new() -> Self {
        Self {
            elapsed: 0.0,
            running: false,
            message: "Press Start to begin timing".to_string(),
        }
    }
}

/// Create a timer application widget
fn timer_ui_builder() -> impl Widget<TimerAppState> {
    // Create widgets
    let time_label = Label::new(|data: &TimerAppState, _env: &druid::Env| {
        format!("Elapsed: {:.2} seconds", data.elapsed)
    })
    .with_text_size(24.0);
    
    let message_label = Label::new(|data: &TimerAppState, _env: &druid::Env| {
        data.message.clone()
    });
    
    let start_button = Button::new("Start")
        .on_click(|ctx, data: &mut TimerAppState, _env| {
            if !data.running {
                data.running = true;
                data.message = "Timer running...".to_string();
                // Request an animation frame to start the timer
                ctx.request_anim_frame();
            }
        });
    
    let stop_button = Button::new("Stop")
        .on_click(|_ctx, data: &mut TimerAppState, _env| {
            data.running = false;
            data.message = "Timer stopped".to_string();
        });
    
    let reset_button = Button::new("Reset")
        .on_click(|ctx, data: &mut TimerAppState, _env| {
            data.elapsed = 0.0;
            data.running = false;
            data.message = "Timer reset".to_string();
            ctx.request_paint(); // Request a repaint to update the display
        });
    
    // Layout the widgets
    Flex::column()
        .with_child(time_label.padding(20.0))
        .with_child(message_label.padding(10.0))
        .with_child(
            Flex::row()
                .with_child(start_button.padding(5.0))
                .with_child(stop_button.padding(5.0))
                .with_child(reset_button.padding(5.0))
        )
        .center()
}

/// Run a timer application with animation frames
pub fn run_timer_app() {
    println!("Starting timer application...");
    println!("The application window will appear shortly.");
    println!("Close the window to return to this console.");
    
    // Create the main window
    let main_window = WindowDesc::new(timer_ui_builder())
        .title("TonleDB Examples - Timer App")
        .window_size((400.0, 200.0));
    
    // Create initial app state
    let initial_state = TimerAppState::new();
    
    // Launch the application
    let launcher = AppLauncher::with_window(main_window);
    let event_sink = launcher.get_external_handle();
    
    launcher
        .configure_env(|env, _data| {
            // Configure the environment if needed
        })
        .launch(initial_state)
        .expect("Failed to launch timer application");
}

/// Example of event-driven programming with custom events
#[derive(Clone, Data, Lens)]
pub struct EventDrivenState {
    pub event_count: i32,
    pub last_event: String,
    pub status: String,
}

/// Custom event type
#[derive(Debug, Clone)]
pub struct CustomEvent {
    pub message: String,
}

impl EventDrivenState {
    pub fn new() -> Self {
        Self {
            event_count: 0,
            last_event: "None".to_string(),
            status: "Ready".to_string(),
        }
    }
}

/// Create an event-driven application widget
fn event_driven_ui_builder() -> impl Widget<EventDrivenState> {
    // Create widgets
    let count_label = Label::new(|data: &EventDrivenState, _env: &druid::Env| {
        format!("Events received: {}", data.event_count)
    });
    
    let last_event_label = Label::new(|data: &EventDrivenState, _env: &druid::Env| {
        format!("Last event: {}", data.last_event)
    });
    
    let status_label = Label::new(|data: &EventDrivenState, _env: &druid::Env| {
        format!("Status: {}", data.status)
    });
    
    let trigger_button = Button::new("Trigger Event")
        .on_click(|ctx, _data: &mut EventDrivenState, _env| {
            // In a real application, you might send a custom event here
            ctx.submit_notification(druid::Selector::new("trigger-event"));
        });
    
    let clear_button = Button::new("Clear")
        .on_click(|_ctx, data: &mut EventDrivenState, _env| {
            data.event_count = 0;
            data.last_event = "None".to_string();
        });
    
    // Layout the widgets
    Flex::column()
        .with_child(count_label.padding(5.0))
        .with_child(last_event_label.padding(5.0))
        .with_child(status_label.padding(5.0))
        .with_child(
            Flex::row()
                .with_child(trigger_button.padding(5.0))
                .with_child(clear_button.padding(5.0))
        )
        .center()
}

/// Run an event-driven application
pub fn run_event_driven_app() {
    println!("Starting event-driven application...");
    println!("The application window will appear shortly.");
    println!("Close the window to return to this console.");
    
    // Create the main window
    let main_window = WindowDesc::new(event_driven_ui_builder())
        .title("TonleDB Examples - Event Driven App")
        .window_size((400.0, 250.0));
    
    // Create initial app state
    let initial_state = EventDrivenState::new();
    
    // Launch the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch event-driven application");
}

/// Example usage of reactive and event-driven functions
pub fn example_usage() {
    println!("Event-Driven and Reactive Examples using Druid");
    println!("=============================================");
    println!("\nNote: These examples will open GUI windows. Close each window");
    println!("to return to the console and see the next example.\n");
    
    println!("1. Simple GUI application:");
    // Note: In a real application, you would run these one at a time
    println!("   To run: call run_simple_gui_app()");
    
    println!("\n2. Timer application:");
    println!("   To run: call run_timer_app()");
    
    println!("\n3. Event-driven application:");
    println!("   To run: call run_event_driven_app()");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state() {
        let mut state = AppState::new();
        assert_eq!(state.counter, 0);
        assert_eq!(state.message, "Welcome to the Druid App!");
        
        state.counter += 1;
        update_timestamp(&mut state);
        assert_eq!(state.counter, 1);
        assert!(!state.last_updated.is_empty());
    }

    #[test]
    fn test_timer_state() {
        let state = TimerAppState::new();
        assert_eq!(state.elapsed, 0.0);
        assert_eq!(state.running, false);
        assert_eq!(state.message, "Press Start to begin timing");
    }

    #[test]
    fn test_event_driven_state() {
        let state = EventDrivenState::new();
        assert_eq!(state.event_count, 0);
        assert_eq!(state.last_event, "None");
        assert_eq!(state.status, "Ready");
    }
}