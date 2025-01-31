use super::Component;
use crate::enums::BroadcastMsg;
use egui::Margin;
use egui_flex::{Flex, FlexAlignContent, FlexItem};
use tokio::sync::mpsc::UnboundedSender;

static SEND_BUTTON_SIZE: f32 = 100.0;

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

    fn update(&mut self, _msg: BroadcastMsg) {}

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.add_space(8.0);

        Flex::horizontal()
            .align_content(FlexAlignContent::Stretch)
            .wrap(false)
            .show(ui, |flex| {
                flex.add_ui(FlexItem::default().grow(4.0), |ui| {
                    let mut text_size = ui.available_size();
                    text_size.x -= SEND_BUTTON_SIZE;

                    ui.add_sized(
                        text_size,
                        egui::TextEdit::multiline(&mut self.input_text)
                            .hint_text("Type here..")
                            .margin(Margin::symmetric(14.0, 18.0)),
                    );
                });

                flex.add_ui(FlexItem::default().basis(SEND_BUTTON_SIZE), |ui| {
                    let send_button =
                        ui.add_sized([90.0, ui.available_height()], egui::Button::new("send"));
                    if send_button.clicked() {
                        println!("send chat msg");
                    }
                });
            });

        ui.add_space(8.0);
    }
}
