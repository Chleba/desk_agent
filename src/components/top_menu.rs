use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::enums::BroadcastMsg;

pub struct TopMenu {
    action_tx: Option<UnboundedSender<BroadcastMsg>>,
    ollama_connected: bool,
}

impl TopMenu {
    pub fn new() -> Self {
        Self {
            action_tx: None,
            ollama_connected: false,
        }
    }
}

impl Component for TopMenu {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn render(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // -- file button menu
                ui.menu_button("Desk Agent", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });
    }

    fn update(&mut self, msg: BroadcastMsg) {
        if let BroadcastMsg::OllamaRunning(r) = msg {
            self.ollama_connected = r.is_ok()
        }
    }

    fn register_tx(&mut self, action_tx: UnboundedSender<BroadcastMsg>) {
        self.action_tx = Some(action_tx);
    }
}
