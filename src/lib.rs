#![warn(clippy::all, rust_2018_idioms)]

mod agents;
mod app;
mod app_state;
mod components;
mod enums;
mod ollama_state;
mod utils;

pub use app::DeskApp;
