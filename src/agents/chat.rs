use crate::{
    components::Component,
    enums::{BroadcastMsg, OllamaModel},
};
use egui::{Frame, Label, RichText, Sense, UiBuilder};
use egui_flex::{Flex, FlexAlignContent, FlexItem};
use tokio::sync::mpsc::UnboundedSender;

use super::Agent;

pub struct ChatAgent {
    action_tx: Option<UnboundedSender<BroadcastMsg>>,
    models: Vec<OllamaModel>,
}

impl ChatAgent {
    pub fn new() -> Self {
        Self {
            action_tx: None,
            models: vec![],
        }
    }
}

impl Agent for ChatAgent {
    fn name(&self) -> &'static str {
        "chat"
    }

    fn description(&self) -> &'static str {
        "Simple chat agent"
    }

    fn set_models(&mut self, models: Vec<OllamaModel>) {
        self.models = models;
    }
}

impl Component for ChatAgent {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn register_tx(&mut self, action_tx: UnboundedSender<BroadcastMsg>) {
        self.action_tx = Some(action_tx);
    }

    fn update(&mut self, _msg: BroadcastMsg) {}

    fn ui(&mut self, ui: &mut egui::Ui) {
        let resp = ui
            .scope_builder(
                UiBuilder::new()
                    .id_salt("chat_agent_component")
                    .sense(Sense::click()),
                |ui| {
                    let resp = ui.response();
                    let visual = ui.style().interact(&resp);
                    // let text_color = visual.text_color();

                    Frame::canvas(ui.style())
                        .fill(visual.bg_fill.linear_multiply(0.3))
                        .stroke(visual.bg_stroke)
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());

                            ui.add_space(32.0);
                            ui.vertical_centered(|ui| {
                                ui.label(self.name().to_string());
                                // Label::new(
                                //     RichText::new(format!("{}", Self::name()))
                                //         .color(text_color)
                                //         .size(32.0),
                                // )
                                // .selectable(false)
                                // .ui(ui);
                            });
                            ui.add_space(32.0);
                        });
                },
            )
            .response;

        if resp.clicked() {
            println!("CHAT CLICKED");
        }

        // ui.add_space(8.0);
        //
        // Flex::horizontal()
        //     .align_content(FlexAlignContent::Stretch)
        //     .wrap(false)
        //     .show(ui, |flex| {
        //         flex.add_ui(FlexItem::default().grow(4.0), |ui| {
        //             let mut text_size = ui.available_size();
        //             text_size.x -= SEND_BUTTON_SIZE;
        //
        //             ui.add_sized(
        //                 text_size,
        //                 egui::TextEdit::multiline(&mut self.input_text)
        //                     .hint_text("Type here..")
        //                     .margin(Margin::symmetric(14.0, 18.0)),
        //             );
        //         });
        //
        //         flex.add_ui(FlexItem::default().basis(SEND_BUTTON_SIZE), |ui| {
        //             let send_button =
        //                 ui.add_sized([90.0, ui.available_height()], egui::Button::new("send"));
        //             if send_button.clicked() {
        //                 println!("send chat msg");
        //             }
        //         });
        //     });
        //
        // ui.add_space(8.0);
    }
}
