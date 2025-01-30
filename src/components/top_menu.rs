use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::enums::BroadcastMsg;

pub struct TopMenu {
    action_tx: Option<UnboundedSender<BroadcastMsg>>,
}

impl TopMenu {
    pub fn new() -> Self {
        Self { action_tx: None }
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
            });
        });
    }

    fn update(&mut self, _msg: BroadcastMsg) {}

    fn register_tx(&mut self, action_tx: UnboundedSender<BroadcastMsg>) {
        self.action_tx = Some(action_tx);
    }
}
