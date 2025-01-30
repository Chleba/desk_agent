use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{components::ollama_settings::OllamaSettings, enums::BroadcastMsg};

pub struct TopMenu {
    // ollama_button: OllamaSettings,
    action_tx: Option<UnboundedSender<BroadcastMsg>>,
}

impl TopMenu {
    pub fn new() -> Self {
        Self {
            // ollama_button: OllamaSettings::new(),
            action_tx: None,
        }
    }
}

impl Component for TopMenu {
    fn render(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // -- file button menu
                ui.menu_button("Desk Assistent", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                // // -- ollama menu button
                // ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                //     self.ollama_button.ui(ui); 
                // });
            });
        });
    }

    fn update(&mut self, msg: BroadcastMsg) {
        // self.ollama_button.update(msg);
    }

    fn register_tx(&mut self, action_tx: UnboundedSender<BroadcastMsg>) {
        // self.ollama_button.register_tx(action_tx.clone());
        self.action_tx = Some(action_tx);
    }
}
