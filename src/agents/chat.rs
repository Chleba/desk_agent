use std::sync::{Arc, Mutex};

use crate::{
    app_state::AppState,
    components::Component,
    enums::{AgentEnum, BroadcastMsg, OllamaModel},
    utils::spawn,
};
use egui::{
    CollapsingHeader, Color32, Frame, Id, KeyboardShortcut, Margin, Modifiers, Sense, UiBuilder,
    Vec2,
};
use egui_flex::{Flex, FlexAlignContent, FlexItem};
use ollama_rs::{
    coordinator::Coordinator,
    generation::{
        chat::ChatMessage,
        tools::implementations::{Calculator, DDGSearcher, Scraper},
    },
    Ollama,
};
use serde::ser::Error;
use tokio::sync::mpsc::UnboundedSender;

use super::Agent;

pub struct ChatAgent {
    action_tx: Option<UnboundedSender<BroadcastMsg>>,
    app_state: Option<Arc<Mutex<AppState>>>,
    models: Vec<OllamaModel>,
    active_model: Option<OllamaModel>,
    history: Vec<ChatMessage>,
    coordinator: Option<
        Arc<
            tokio::sync::Mutex<Coordinator<Vec<ChatMessage>, (DDGSearcher, (Scraper, Calculator))>>,
        >,
    >,
    sys_msg: String,
}

impl ChatAgent {
    pub fn new() -> Self {
        Self {
            action_tx: None,
            app_state: None,
            models: vec![],
            active_model: None,
            history: vec![],
            coordinator: None,
            sys_msg: "".to_string(),
        }
    }

    fn msg_to_coordinator(&mut self, msg: ChatMessage) {
        if let Some(coordinator) = self.coordinator.clone() {
            let action_tx = self.action_tx.clone();
            let sys_msg = self.sys_msg.clone();

            tokio::spawn(async move {
                let _ = Self::send_chat_msg(action_tx, coordinator, msg.clone(), sys_msg).await;
            });
            // spawn(Self::send_chat_msg(
            //     self.action_tx.clone(),
            //     coordinator,
            //     msg.clone(),
            //     self.sys_msg.clone(),
            // ));
        }
    }

    async fn send_chat_msg(
        action_tx: Option<UnboundedSender<BroadcastMsg>>,
        coordinator: Arc<
            tokio::sync::Mutex<Coordinator<Vec<ChatMessage>, (DDGSearcher, (Scraper, Calculator))>>,
        >,
        msg: ChatMessage,
        sys_msg: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // ) {
        let mut msgs = vec![];
        if !sys_msg.trim().is_empty() {
            let sys_chat_msg =
                ChatMessage::new(ollama_rs::generation::chat::MessageRole::System, sys_msg);
            msgs.push(sys_chat_msg);
        }
        msgs.push(msg.clone());

        println!("USER: {}", msg.content.clone());
        let resp = coordinator.lock().await.chat(msgs).await?;

        // if let Some(tx) = action_tx {
        //     if let Ok(r) = resp {
        //         let _ = tx.send(BroadcastMsg::GetChatReponse(r.message.clone()));
        //     }
        // }
        // println!("{:?} CHAT RESPONSE", resp);
        Ok(())
    }

    fn change_active_model(&mut self) {
        if let Some(active_model) = self.active_model.clone() {
            self.history.clear();
            self.coordinator = Some(self.get_coordinator(
                active_model.clone(),
                self.history.clone(),
                self.app_state.clone(),
                self.action_tx.clone(),
            ));
        }
    }

    fn advanced_ui(&mut self, ui: &mut egui::Ui) {
        CollapsingHeader::new("advanced options:")
            .default_open(false)
            .show(ui, |ui| {
                ui.small("system message:");
                ui.horizontal(|ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.sys_msg)
                            .return_key(KeyboardShortcut::new(Modifiers::SHIFT, egui::Key::Enter))
                            .desired_rows(2)
                            .hint_text("Type here..")
                            .margin(Margin::symmetric(4.0, 4.0)),
                    );
                    if ui.button("save").clicked() {
                        println!("save system message: {}", self.sys_msg.clone());
                    }
                });
            });
    }

    fn grid_ui(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("")
            .num_columns(2)
            .spacing(egui::Vec2 { x: 4.0, y: 0.0 })
            .show(ui, |ui| {
                // --------
                ui.small("agent:");
                ui.label(self.name().to_string());
                ui.end_row();

                // --------
                let previous_value = self.active_model.clone();
                ui.small("model:");
                if let Some(m) = self.active_model.clone() {
                    egui::ComboBox::from_id_salt(Id::new(format!("{}_combo", m.name)))
                        .width(100.0)
                        .truncate()
                        .selected_text(m.name)
                        .show_ui(ui, |ui| {
                            for om in self.models.iter() {
                                ui.selectable_value(
                                    &mut self.active_model,
                                    Some(om.clone()),
                                    &om.name,
                                );
                            }
                        });

                    if self.active_model != previous_value {
                        self.change_active_model();
                    }
                }
                ui.end_row();

                // --------
                ui.small("");
                self.advanced_ui(ui);
                ui.end_row();
            });
    }
}

impl Agent for ChatAgent {
    fn name(&self) -> &'static str {
        "chat"
    }

    fn description(&self) -> &'static str {
        "Simple chat agent"
    }

    fn agent(&self) -> AgentEnum {
        AgentEnum::Chat
    }

    fn select_agent(&mut self, agent: AgentEnum) {
        if agent.eq(&self.agent()) {
            if let Some(app_state) = self.app_state.clone() {
                let models = app_state.lock().unwrap().ollama_state.models.clone();
                if !models.is_empty() {
                    let model = models[0].clone();
                    self.coordinator = Some(self.get_coordinator(
                        model,
                        self.history.clone(),
                        self.app_state.clone(),
                        self.action_tx.clone(),
                    ));
                }
            }
        } else {
            self.coordinator = None;
        }
    }
}

impl Component for ChatAgent {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn register_app_state(&mut self, app_state: Arc<Mutex<AppState>>) {
        self.app_state = Some(app_state);
    }

    fn register_tx(&mut self, action_tx: UnboundedSender<BroadcastMsg>) {
        self.action_tx = Some(action_tx);
    }

    fn update(&mut self, msg: BroadcastMsg) {
        match msg {
            BroadcastMsg::SelectAgent(agent) => {
                self.select_agent(agent);
            }
            BroadcastMsg::OllamaModels(models) => {
                self.models = models.clone();
                if !self.models.is_empty() {
                    if self.active_model.is_none() {
                        self.active_model = Some(models[0].clone());
                    } else {
                        let active_model = self.active_model.clone().unwrap();
                        if !self.models.contains(&active_model) {
                            self.active_model = Some(models[0].clone());
                        }
                    }

                    if let Some(aps) = self.app_state.clone() {
                        let agent = aps.lock().unwrap().active_agent.clone();
                        self.select_agent(agent);
                    }
                }
            }
            BroadcastMsg::SendUserMessage(msg) => {
                self.msg_to_coordinator(msg);
            }
            _ => {}
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        let resp = ui
            .scope_builder(
                UiBuilder::new()
                    .id_salt("chat_agent_component")
                    .sense(Sense::click()),
                |ui| {
                    let resp = ui.response();
                    let visual = ui.style().interact(&resp);

                    Frame::canvas(ui.style())
                        .fill(visual.bg_fill.linear_multiply(0.3))
                        .stroke(visual.bg_stroke)
                        .show(ui, |ui| {
                            Flex::horizontal()
                                .align_content(FlexAlignContent::Stretch)
                                .w_full()
                                .show(ui, |flex| {
                                    flex.add_ui(FlexItem::new().grow(0.8), |ui| {
                                        // flex.add_ui(FlexItem::new().basis(200.0), |ui| {
                                        self.grid_ui(ui);
                                    });

                                    flex.add_ui(FlexItem::new().basis(25.0), |ui| {
                                        // flex.add_ui(FlexItem::new().grow(0.1), |ui| {
                                        if let Some(app_state) = self.app_state.clone() {
                                            let active_agent =
                                                app_state.lock().unwrap().active_agent.clone();
                                            if active_agent.eq(&self.agent()) {
                                                egui::Frame::default()
                                                    .stroke(
                                                        ui.visuals()
                                                            .widgets
                                                            .noninteractive
                                                            .bg_stroke,
                                                    )
                                                    .rounding(
                                                        ui.visuals()
                                                            .widgets
                                                            .noninteractive
                                                            .rounding,
                                                    )
                                                    .fill(Color32::from_rgb(200, 200, 0))
                                                    .show(ui, |ui| {
                                                        ui.set_width(ui.available_width());
                                                        // ui.set_max_height(50.0);
                                                        ui.set_height(50.0);
                                                        ui.label("");
                                                    });
                                            }
                                        }
                                    });
                                });
                            ui.end_row();
                        });
                },
            )
            .response;

        if resp.clicked() {
            self.send_selected_agent(self.action_tx.clone());
        }
    }
}
