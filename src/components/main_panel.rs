use super::{chat_input::ChatInput, Component};
use crate::{components::ollama_settings::OllamaSettings, enums::BroadcastMsg};
use eframe::Frame;
use egui::{Color32, ScrollArea};
use egui_flex::{Flex, FlexAlign, FlexAlignContent, FlexItem};
use tokio::sync::mpsc::UnboundedSender;

pub struct MainPanel {
    ollama_button: OllamaSettings,
    chat_input: ChatInput,

    action_tx: Option<UnboundedSender<BroadcastMsg>>,
}

impl MainPanel {
    pub fn new() -> Self {
        Self {
            ollama_button: OllamaSettings::new(),
            chat_input: ChatInput::new(),
            action_tx: None,
        }
    }
}

impl Component for MainPanel {
    fn register_tx(&mut self, action_tx: UnboundedSender<BroadcastMsg>) {
        self.ollama_button.register_tx(action_tx.clone());
        self.chat_input.register_tx(action_tx.clone());

        self.action_tx = Some(action_tx);
    }

    fn update(&mut self, msg: BroadcastMsg) {
        self.ollama_button.update(msg.clone());
        self.chat_input.update(msg);
    }

    fn render(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // let frame = egui::Frame::group(ui.style());

            ui.horizontal(|ui| {
                // -- ollama menu button
                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    self.ollama_button.ui(ui);
                });
            });

            ui.add_space(8.0);

            // Flex::vertical()
            //     .align_content(FlexAlignContent::SpaceBetween)
            //     .wrap(true)
            //     .h_full()
            //     .w_full()
            //     .show(ui, |flex| {
            //         // flex.add_ui(FlexItem::new().frame(frame).basis(20.0), |ui| {
            //         flex.add_ui(FlexItem::new().basis(20.0), |ui| {
            //             ui.vertical(|ui| {
            //                 ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
            //                     self.ollama_button.ui(ui);
            //                 });
            //             });
            //         });

            //         flex.add_ui(FlexItem::new().basis(8.0), |ui| {
            //             ui.add_space(8.0);
            //         });

            //         // flex.add_flex(
            //         //     FlexItem::new().grow(2.0).frame(frame),
            //         //     Flex::horizontal()
            //         //         .w_full()
            //         //         // .h_full()
            //         //         .align_items(FlexAlign::Stretch),
            //         //     |flex| {
            //         //         flex.add_ui(FlexItem::new().frame(frame).grow(1.0), |ui| {
            //         //             ScrollArea::vertical()
            //         //                 .animated(false)
            //         //                 .max_height(500.0)
            //         //                 .auto_shrink([false, false])
            //         //                 .stick_to_bottom(true)
            //         //                 .show(ui, |ui| {
            //         //                     ui.label("MASLO FRAME CENTER");
            //         //                 });

            //         //             ui.label("maslo");
            //         //         });
            //         //     },
            //         // );

            //         flex.add_ui(FlexItem::new().frame(frame).grow(2.0), |ui| {
            //             egui::Frame::default()
            //                 .stroke(egui::epaint::Stroke {
            //                     color: Color32::from_rgb(60, 60, 60),
            //                     width: 1.0,
            //                 })
            //                 .rounding(egui::epaint::Rounding::same(4.0))
            //                 .show(ui, |ui| {
            //                     println!("{:?} - MAX RECT GIGGA", ui.max_rect());

            //             ScrollArea::vertical()
            //                 .animated(false)
            //                 // .max_height(ui.max().y)
            //                 .auto_shrink([false, false])
            //                 .stick_to_bottom(true)
            //                 .show(ui, |ui| {
            //                     ui.label("MASLO FRAME CENTER");
            //                 });

            //                     ui.label("maslo");
            //                 });
            //         });

            //         flex.add_ui(FlexItem::new().basis(8.0), |ui| {
            //             ui.add_space(8.0);
            //         });

            //         flex.add_ui(FlexItem::new().frame(frame).basis(80.0), |ui| {
            //             ui.label("maslo");
            //             // egui::Frame::default()
            //             //     .stroke(egui::epaint::Stroke {
            //             //         color: Color32::from_rgb(60, 60, 60),
            //             //         width: 1.0,
            //             //     })
            //             //     .rounding(egui::epaint::Rounding::same(4.0))
            //             //     .show(ui, |ui| {
            //             //         ui.label("maslo");
            //             //     });
            //         });

            //         // flex.add_flex(FlexItem::new().frame(frame), Flex::horizontal().w_full().h_full(), |flex| {
            //         //     flex.add_ui(item, content)
            //         // });
            //     });

            ui.vertical_centered_justified(|ui| {
            //     Flex::vertical()
            //         .align_content(FlexAlignContent::End)
            //         .w_full()
            //         // .height_percent(1.5)
            //         .show(ui, |flex| {
            //             flex.add_ui(FlexItem::new().grow(0.4), |ui| {
            //                 ui.vertical(|ui| {
                                egui::Frame::default()
                                    // .stroke(egui::epaint::Stroke {
                                    //     color: Color32::from_rgb(60, 60, 60),
                                    //     width: 1.0,
                                    // })
                                    // .rounding(egui::epaint::Rounding::same(4.0))
                                    .show(ui, |ui| {
                                        ScrollArea::vertical()
                                            .animated(false)
                                            // .max_height(800.0)
                                            .auto_shrink([false, false])
                                            .stick_to_bottom(true)
                                            .show(ui, |ui| {
                                                ui.label("MASLO FRAME CENTER");
                                            });
                                    });
                            // });
            //             });

            //             flex.add_ui(FlexItem::new(), |ui| {
            //                 ui.separator();
            //             });

            //             flex.add_ui(FlexItem::new().grow(1.0), |ui| {
            //                 ui.set_height(200.0);
            //                 // ui.separator();
            //                 self.chat_input.ui(ui);
            //             });
            //         });
            });
            // ui.separator();

            // self.chat_input.ui(ui);
        });
    }
}
