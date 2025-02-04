use crate::{
    components::Component,
    enums::{AgentEnum, BroadcastMsg},
};
use std::any::Any;
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
}
