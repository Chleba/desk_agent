use super::Component;
use crate::{
    agents::{chat::ChatAgent, Agent, AgentComponent},
    enums::{BroadcastMsg, OllamaModel},
};
use tokio::sync::mpsc::UnboundedSender;

pub struct AgentPanel {
    action_tx: Option<UnboundedSender<BroadcastMsg>>,
    models: Vec<OllamaModel>,
    // agents: Vec<Box<dyn Component + Agent>>,
    // agents: Vec<Box<dyn Component>>,
    agents: Vec<Box<dyn AgentComponent>>,
}

impl AgentPanel {
    pub fn new() -> Self {
        let chat_agent = ChatAgent::new();

        Self {
            action_tx: None,
            models: vec![],
            agents: vec![Box::new(chat_agent)],
        }
    }

    fn add_models_to_agens(&mut self) {
        for agent in self.agents.iter_mut() {
            agent.set_models(self.models.clone());
            // if let Some(a) = agent.as_any().downcast_ref::<&mut dyn Agent>() {
            //     a.set_models(self.models.clone());
            // }
            // let agent = agent.as_ref() as &dyn Agent;
            // agent.set_models(self.models.clone());
            // // agent.register_tx(action_tx.clone());
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
