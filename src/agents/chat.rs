use std::sync::{Arc, Mutex};

use crate::{
    app_state::AppState,
    components::Component,
    enums::{AgentEnum, BroadcastMsg, OllamaModel},
    utils::spawn,
};
use egui::{Color32, Frame, Id, Sense, UiBuilder};
use egui_flex::{Flex, FlexAlignContent, FlexItem};
use ollama_rs::{coordinator::Coordinator, generation::chat::ChatMessage, Ollama};
use tokio::sync::mpsc::UnboundedSender;

use super::Agent;

pub struct ChatAgent {
    action_tx: Option<UnboundedSender<BroadcastMsg>>,
    app_state: Option<Arc<Mutex<AppState>>>,
    models: Vec<OllamaModel>,
    active_model: Option<OllamaModel>,
    history: Vec<ChatMessage>,
    coordinator: Option<Arc<tokio::sync::Mutex<Coordinator<Vec<ChatMessage>, ()>>>>,
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
        }
    }

    fn msg_to_coordinator(&mut self, msg: ChatMessage) {
        if let Some(coordinator) = self.coordinator.clone() {
            spawn(Self::send_chat_msg(
                self.action_tx.clone(),
                coordinator,
                msg.clone(),
            ));
        }
    }

    async fn send_chat_msg(
        action_tx: Option<UnboundedSender<BroadcastMsg>>,
        coordinator: Arc<tokio::sync::Mutex<Coordinator<Vec<ChatMessage>, ()>>>,
        msg: ChatMessage,
    ) {
        let resp = coordinator.lock().await.chat(vec![msg]).await.unwrap();

        if let Some(tx) = action_tx {
            let _ = tx.send(BroadcastMsg::GetChatReponse(resp.message.clone()));
        }
        println!("{:?} CHAT RESPONSE", resp);
    }

    fn change_active_model(&mut self) {
        let ollama = Ollama::default();
        if let Some(active_model) = self.active_model.clone() {
            let model = active_model.name.clone();
            self.coordinator = Some(Arc::new(tokio::sync::Mutex::new(Coordinator::new(
                ollama,
                model,
                self.history.clone(),
            ))));
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

    fn agent(&self) -> AgentEnum {
        AgentEnum::Chat
    }

    fn select_agent(&mut self, agent: AgentEnum) {
        if agent.eq(&self.agent()) {
            if let Some(app_state) = self.app_state.clone() {
                let models = app_state.lock().unwrap().ollama_state.models.clone();
                if !models.is_empty() {
                    let ollama = Ollama::default();
                    let model = models[0].name.clone();
                    self.coordinator = Some(Arc::new(tokio::sync::Mutex::new(Coordinator::new(
                        ollama,
                        model,
                        self.history.clone(),
                    ))));
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

        if let Some(aps) = self.app_state.clone() {
            let agent = aps.lock().unwrap().active_agent.clone();
            self.select_agent(agent);
        }
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
                if !self.models.is_empty() && self.active_model.is_none() {
                    self.active_model = Some(models[0].clone());
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
                                    flex.add_ui(FlexItem::new().grow(1.0), |ui| {
                                        egui::Grid::new("")
                                            .num_columns(2)
                                            .spacing(egui::Vec2 { x: 4.0, y: 0.0 })
                                            .show(ui, |ui| {
                                                ui.small("agent:");
                                                ui.label(self.name().to_string());
                                                ui.end_row();

                                                let previous_value = self.active_model.clone();

                                                ui.small("model:");
                                                if let Some(m) = self.active_model.clone() {
                                                    egui::ComboBox::from_id_salt(Id::new(format!(
                                                        "{}_combo",
                                                        m.name
                                                    )))
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
                                            });
                                    });

                                    flex.add_ui(FlexItem::new().basis(25.0), |ui| {
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
