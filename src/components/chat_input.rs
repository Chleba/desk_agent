use super::Component;
use crate::{components::ollama_settings::OllamaSettings, enums::BroadcastMsg};
use egui::{Color32, ScrollArea};
use tokio::sync::mpsc::UnboundedSender;

pub struct ChatInput {
    action_tx: Option<UnboundedSender<BroadcastMsg>>,
    pub input_text: String,
}

impl ChatInput {
    pub fn new() -> Self {
        Self {
            action_tx: None,
            input_text: String::new(),
        }
    }
}

impl Component for ChatInput {
    fn register_tx(&mut self, action_tx: UnboundedSender<BroadcastMsg>) {
        self.action_tx = Some(action_tx);
    }

    fn update(&mut self, _msg: BroadcastMsg) {
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {

            let input = egui::TextEdit::multiline(&mut self.input_text)
                .hint_text("Type here..")
                .show(ui);

        });
    }

    // fn render(&mut self, ctx: &egui::Context) {
    //     egui::CentralPanel::default().show(ctx, |ui| {
    //         ui.horizontal(|ui| {
    //             // -- ollama menu button
    //             ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
    //                 self.ollama_button.ui(ui);
    //             });
    //         });

    //         ui.separator();
    //         ui.add_space(8.0);

    //         ui.vertical(|ui| {
    //             egui::Frame::default()
    //                 .stroke(egui::epaint::Stroke {
    //                     color: Color32::from_rgb(100, 100, 100),
    //                     width: 1.0,
    //                 })
    //                 .rounding(egui::epaint::Rounding::same(4.0))
    //                 .show(ui, |ui| {
    //                     ScrollArea::vertical()
    //                         .animated(false)
    //                         .max_height(500.0)
    //                         .auto_shrink([false, false])
    //                         .stick_to_bottom(true)
    //                         .show(ui, |ui| {
    //                             ui.label("MASLO FRAME CENTER");
    //                         });
    //                 });
    //         });
    //     });
    // }
}
