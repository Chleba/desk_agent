use super::{chat_input::ChatInput, Component};
use crate::{components::ollama_settings::OllamaSettings, enums::BroadcastMsg};
// use egui::{Align, Color32, Layout, RichText, ScrollArea, Widget};
use eframe::emath::Vec2;
use egui::{
    Align, Frame, Label, Layout, Rect, RichText, Rounding, ScrollArea, Shape, Stroke, Ui,
    UiBuilder, Widget,
};
use egui_infinite_scroll::InfiniteScroll;
use ollama_rs::generation::chat::{ChatMessage, MessageRole};
use tokio::sync::mpsc::UnboundedSender;

pub struct Messages {
    action_tx: Option<UnboundedSender<BroadcastMsg>>,
    messages: InfiniteScroll<ChatMessage, usize>,
}

impl Messages {
    pub fn new() -> Self {
        let mut infinite_scroll = InfiniteScroll::new();
        infinite_scroll.virtual_list.hide_on_resize(None);

        Self {
            action_tx: None,
            messages: infinite_scroll.start_loader(move |cursor, cb| {
                println!("loading messages");
            }),
        }
    }
}

impl Component for Messages {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn init(&mut self) {}

    fn register_tx(&mut self, action_tx: UnboundedSender<BroadcastMsg>) {
        self.action_tx = Some(action_tx);
    }

    fn update(&mut self, msg: BroadcastMsg) {
        match msg {
            BroadcastMsg::SendUserMessage(m) => {
                self.messages.items.push(m);
            }
            BroadcastMsg::GetChatReponse(m) => {
                self.messages.items.push(m);
            }
            _ => {}
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        self.messages.ui(ui, 5, |ui, _index, item| {
            let layout = if item.role == MessageRole::User {
                Layout::top_down(Align::Max)
            } else {
                Layout::top_down(Align::Min)
            };

            let max_msg_width = ui.available_width() - 40.0;

            ui.with_layout(layout, |ui| {
                ui.set_max_width(max_msg_width);

                let mut measure = |text| {
                    let label = egui::Label::new(text);
                    let (_pos, galley, _response) = label.layout_in_ui(
                        &mut ui.new_child(egui::UiBuilder::new().max_rect(ui.max_rect())),
                    );
                    let rect = galley.rect;
                    f32::min(rect.width() + 8.0 * 2.0 + 8.0 * 2.0 + 0.1, max_msg_width)
                };

                let content = RichText::new(&item.content);
                let mut msg_width = measure(content.clone());

                ui.set_min_width(msg_width);

                let msg_color = if item.role == MessageRole::User {
                    ui.style().visuals.widgets.inactive.bg_fill
                } else {
                    ui.style().visuals.extreme_bg_color
                };

                let rounding = 8.0;
                let margin = 8.0;
                let response = Frame::none()
                    .rounding(Rounding {
                        ne: if item.role == MessageRole::User {
                            0.0
                        } else {
                            rounding
                        },
                        nw: if item.role == MessageRole::User {
                            rounding
                        } else {
                            0.0
                        },
                        se: rounding,
                        sw: rounding,
                    })
                    .inner_margin(margin)
                    .outer_margin(margin)
                    .fill(msg_color)
                    .show(ui, |ui| {
                        ui.with_layout(Layout::top_down(Align::Min), |ui| {
                            // if let Some(from) = name {
                            //     Label::new(from).ui(ui);
                            // }

                            ui.label(&item.content);
                        });
                    })
                    .response;

                let points = if item.role == MessageRole::User {
                    let top = response.rect.right_top() + Vec2::new(-margin, margin);
                    let arrow_rect = Rect::from_two_pos(top, top + Vec2::new(rounding, rounding));

                    vec![
                        arrow_rect.left_top(),
                        arrow_rect.right_top(),
                        arrow_rect.left_bottom(),
                    ]
                } else {
                    let top = response.rect.left_top() + Vec2::splat(margin);
                    let arrow_rect = Rect::from_two_pos(top, top + Vec2::new(-rounding, rounding));

                    vec![
                        arrow_rect.left_top(),
                        arrow_rect.right_top(),
                        arrow_rect.right_bottom(),
                    ]
                };

                ui.painter()
                    .add(Shape::convex_polygon(points, msg_color, Stroke::NONE))
            });
        });
    }
}
