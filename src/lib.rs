#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod components;
mod ollama_state;
mod app_state;
mod enums;
mod utils;

pub use app::DeskApp;
