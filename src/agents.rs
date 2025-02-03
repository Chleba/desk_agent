use std::any::Any;
// use tokio::sync::mpsc::UnboundedSender;

use crate::{components::Component, enums::OllamaModel};

pub mod chat;
pub mod websearch;

pub trait AgentComponent: Component + Agent {}
impl<T: ?Sized + Component + Agent> AgentComponent for T {}

pub trait Agent: Any {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;

    #[allow(unused_variables)]
    fn set_models(&mut self, models: Vec<OllamaModel>) {}
}
