use std::sync::{Arc, Mutex};

use crate::{
    app_state::AppState,
    components::{
        agents_panel::AgentPanel, bottom_panel::ChatBottomPanel, main_panel::MainPanel,
        top_menu::TopMenu, Component,
    },
    enums::BroadcastMsg,
};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

pub struct DeskApp {
    app_state: Arc<Mutex<AppState>>,
    components: Vec<Box<dyn Component>>,
    action_tx: UnboundedSender<BroadcastMsg>,
    action_rx: UnboundedReceiver<BroadcastMsg>,
}

impl DeskApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (action_tx, action_rx) = mpsc::unbounded_channel();

        let app_state = Arc::new(Mutex::new(AppState::new(cc)));

        let top_menu = TopMenu::new();
        let main_panel = MainPanel::new();
        let agent_panel = AgentPanel::new();
        let bottom_panel = ChatBottomPanel::new();

        Self {
            action_rx,
            action_tx,
            app_state,
            components: vec![
                Box::new(top_menu),
                Box::new(agent_panel),
                // -- add bottom before center for correct scrollview height
                Box::new(bottom_panel),
                Box::new(main_panel),
            ],
        }
    }

    pub fn init(&mut self) {
        self.init_components();
        self.register_tx();
        self.register_app_state();
        {
            self.app_state.lock().unwrap().init();
        }
    }

    fn init_components(&mut self) {
        for component in self.components.iter_mut() {
            component.init();
        }
    }

    fn register_app_state(&mut self) {
        for component in self.components.iter_mut() {
            component.register_app_state(self.app_state.clone());
        }
    }

    fn register_tx(&mut self) {
        let action_tx = &self.action_tx;

        {
            self.app_state
                .lock()
                .unwrap()
                .register_tx(action_tx.clone());
        }

        for component in self.components.iter_mut() {
            component.register_tx(action_tx.clone());
        }
    }
}

impl eframe::App for DeskApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        {
            self.app_state.lock().unwrap().save(storage);
        }
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let action_rx = &mut self.action_rx;

        while let Ok(msg) = action_rx.try_recv() {
            {
                self.app_state.lock().unwrap().update(msg.clone());
            }
            for component in self.components.iter_mut() {
                component.update(msg.clone());
            }
        }

        // -- set font size for whole app
        ctx.set_pixels_per_point(1.2);

        // -- render components
        for component in self.components.iter_mut() {
            component.render(ctx);
        }
    }
}
