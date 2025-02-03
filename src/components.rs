use std::any::Any;
use tokio::sync::mpsc::UnboundedSender;

use crate::enums::BroadcastMsg;

pub mod agents_panel;
pub mod bottom_panel;
pub mod chat_input;
pub mod main_panel;
pub mod ollama_settings;
pub mod top_menu;

pub trait Component: Any {
    fn init(&mut self) {}

    #[allow(unused_variables)]
    fn as_any(&self) -> &dyn Any;

    #[allow(unused_variables)]
    fn register_tx(&mut self, action_tx: UnboundedSender<BroadcastMsg>) {}

    #[allow(unused_variables)]
    fn render(&mut self, ctx: &egui::Context) {}

    #[allow(unused_variables)]
    fn ui(&mut self, ui: &mut egui::Ui) {}

    #[allow(unused_variables)]
    fn update(&mut self, msg: BroadcastMsg) {}
}
