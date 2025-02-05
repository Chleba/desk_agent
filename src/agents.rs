use crate::{
    app_state::{self, AppState},
    components::Component,
    enums::{AgentEnum, BroadcastMsg, OllamaModel},
};
use ollama_rs::{coordinator::Coordinator, generation::chat::ChatMessage, Ollama};
use std::{
    any::Any,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc::UnboundedSender;

pub mod chat;
pub mod websearch;

pub trait AgentComponent: Component + Agent {}
impl<T: ?Sized + Component + Agent> AgentComponent for T {}

pub trait Agent: Any {
    fn name(&self) -> &'static str;
    #[allow(dead_code)]
    fn description(&self) -> &'static str;
    fn agent(&self) -> AgentEnum;

    #[allow(unused_variables)]
    fn select_agent(&mut self, agent: AgentEnum) {}

    fn send_selected_agent(&mut self, action: Option<UnboundedSender<BroadcastMsg>>) {
        if let Some(tx) = action {
            let _ = tx.send(BroadcastMsg::SelectAgent(self.agent()));
        }
    }

    fn get_coordinator(
        &mut self,
        active_model: OllamaModel,
        history: Vec<ChatMessage>,
        app_state: Option<Arc<Mutex<AppState>>>,
        action_tx: Option<UnboundedSender<BroadcastMsg>>,
    ) -> Arc<tokio::sync::Mutex<Coordinator<Vec<ChatMessage>, ()>>> {
        let (url, port) = self.get_ollama_url(app_state);

        let ollama = Ollama::new(url, port);
        let model = active_model.name.clone();
        let coordinator = Arc::new(tokio::sync::Mutex::new(Coordinator::new(
            ollama,
            model,
            history.clone(),
        )));

        if let Some(tx) = action_tx.clone() {
            let _ = tx.send(BroadcastMsg::SelectAgentModel(active_model));
        }

        coordinator
    }

    fn get_ollama_url(&mut self, app_state: Option<Arc<Mutex<AppState>>>) -> (String, u16) {
        if let Some(state) = app_state.clone() {
            let url = state.lock().unwrap().ollama_state.url.clone();
            if let Some((base_url, port)) = url.rsplit_once(':') {
                if let Ok(port_num) = port.parse::<u16>() {
                    return (base_url.to_string(), port_num);
                }
            }
        }
        ("http://localhost/".to_string(), 11343)
    }
}
