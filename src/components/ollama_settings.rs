use egui::{popup_below_widget, FontId, Id, PopupCloseBehavior, RichText, TextEdit};
use egui_flex::{Flex, FlexAlignContent, FlexItem};
use egui_form::{
    garde::{field_path, GardeReport},
    Form, FormField,
};
use egui_inbox::broadcast::Broadcast;
use garde::Validate;
// use futures::channel::mpsc::UnboundedSender;
use tokio::sync::mpsc::UnboundedSender;

use crate::enums::BroadcastMsg;

use super::Component;

#[derive(Debug, Validate)]
struct OllamaURL {
    #[garde(length(min = 2, max = 150))]
    url: String,
}

pub struct OllamaSettings {
    url: OllamaURL,
    // broadcast: Option<Broadcast<BroadcastMsg>>,
    action_tx: Option<UnboundedSender<BroadcastMsg>>,
}

impl OllamaSettings {
    pub fn new() -> Self {
        Self {
            url: OllamaURL { url: String::new() },
            // broadcast: None,
            action_tx: None,
        }
    }
}

impl Component for OllamaSettings {
    fn update(&mut self, msg: BroadcastMsg) {
        println!("{:?} OllamaSettings msg", msg);
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        let lamma_img = egui::Image::new(egui::include_image!("../../assets/ollama.png"));

        let button = egui::Button::image_and_text(lamma_img, "Ollama");
        let button_id = Id::new("ollama_popup");
        let button_res = ui.add(button);

        if button_res.clicked() {
            ui.memory_mut(|mem| {
                println!("OPEN SETTINGS");
                mem.toggle_popup(button_id);
            });
        }

        popup_below_widget(
            ui,
            button_id,
            &button_res,
            PopupCloseBehavior::CloseOnClickOutside,
            |ui| {
                // println!("OPEN SETTINGS");

                // ui.set_min_width(310.0);
                ui.set_width(310.0);

                let mut form = Form::new().add_report(GardeReport::new(self.url.validate()));

                Flex::horizontal()
                    .align_content(FlexAlignContent::Stretch)
                    .w_full()
                    .show(ui, |flex| {
                        flex.add_ui(FlexItem::new().grow(1.0), |ui| {
                            FormField::new(&mut form, field_path!("url"))
                                .label("Ollama URL:")
                                .ui(ui, TextEdit::singleline(&mut self.url.url));
                        });

                        let broadcast = self.action_tx.clone();

                        flex.add_ui(FlexItem::new().grow(1.0), |ui| {
                            if let Some(Ok(())) = form.handle_submit(&ui.button("save"), ui) {

                                if let Some(b) = broadcast {
                                    println!("{} - submit",self.url.url);
                                    let _ = b.send(BroadcastMsg::SetOllamaURL(self.url.url.clone()));
                                }

                                println!("maslo save ollama url");
                            }
                        });
                    });
            },
        );
    }

    fn register_tx(&mut self, action_tx: UnboundedSender<BroadcastMsg>) {
    // fn register_tx(&mut self, broadcast: Broadcast<BroadcastMsg>) {
        // self.broadcast = Some(broadcast);
        self.action_tx = Some(action_tx);
    }
}
