use super::Component;
use crate::{
    agents::{
        chat::ChatAgent,
        websearch::{self, WebSearchAgent},
        Agent, AgentComponent,
    },
    enums::{BroadcastMsg, OllamaModel},
};
use tokio::sync::mpsc::UnboundedSender;

pub struct AgentPanel {
    action_tx: Option<UnboundedSender<BroadcastMsg>>,
    models: Vec<OllamaModel>,
    agents: Vec<Box<dyn AgentComponent>>,
}

impl AgentPanel {
    pub fn new() -> Self {
        let chat_agent = ChatAgent::new();
        let websearch_agent = WebSearchAgent::new();

        Self {
            action_tx: None,
            models: vec![],
            agents: vec![Box::new(chat_agent), Box::new(websearch_agent)],
        }
    }

    fn add_models_to_agens(&mut self) {
        for agent in self.agents.iter_mut() {
            agent.set_models(self.models.clone());
        }
    }
}

impl Component for AgentPanel {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn init(&mut self) {
        for agent in self.agents.iter_mut() {
            agent.init();
        }
    }

    fn register_tx(&mut self, action_tx: UnboundedSender<BroadcastMsg>) {
        for agent in self.agents.iter_mut() {
            agent.register_tx(action_tx.clone());
        }
        self.action_tx = Some(action_tx);
    }

    fn update(&mut self, msg: BroadcastMsg) {
        for agent in self.agents.iter_mut() {
            agent.update(msg.clone());
        }
        match msg {
            BroadcastMsg::OllamaModels(models) => {
                self.models = models;
                self.add_models_to_agens();
            }
            _ => {}
        }
    }

    fn render(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("agent_panel").show(ctx, |ui| {
            ui.label("Agents:");

            ui.vertical(|ui| {
                for agent in self.agents.iter_mut() {
                    agent.ui(ui);
                }
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}
