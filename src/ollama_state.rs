use crate::{
    enums::{BroadcastMsg, OllamaModel, OllamaTagsResult},
    utils::spawn,
};
use egui_inbox::broadcast::{self, Broadcast};
use futures::TryFutureExt;

#[derive(serde::Deserialize, Default, serde::Serialize, Debug, Clone)]
pub struct OllamaState {
    #[serde(skip)]
    broadcast: Option<Broadcast<BroadcastMsg>>,
    url: String,
    models: Vec<OllamaModel>,
}

static OLLAMA_STATE_KEY: &str = "ollama_state";

impl OllamaState {
    pub fn new(cc: &eframe::CreationContext<'_>, url: String) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, OLLAMA_STATE_KEY).unwrap_or_default();
        }

        Self {
            broadcast: None,
            url,
            models: vec![],
        }
    }

    pub fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, OLLAMA_STATE_KEY, self);
    }

    pub fn init(&mut self, broadcast: Broadcast<BroadcastMsg>) {
        // spawn(Self::get_tags(self.url.clone(), broadcast.clone()));

        self.broadcast = Some(broadcast);
    }

    pub fn update(&mut self, msg: BroadcastMsg) {
        match msg {
            BroadcastMsg::OllamaMsg(bm) => {
                println!("{:?}", bm);
            },
            BroadcastMsg::SetOllamaURL(url) => {
                self.url = url;
            }
        }
    }

    pub fn set_ollama_url(&mut self, url: String) {
        self.url = url;
    }

    async fn get_tags(url: String, broadcast: Broadcast<BroadcastMsg>) {
        let tags: OllamaTagsResult = reqwest::get(format!("{}/api/tags", url))
            .and_then(reqwest::Response::json)
            .await
            .unwrap();

        broadcast.send(BroadcastMsg::OllamaMsg(OllamaState {
            url,
            models: tags.clone().models,
            broadcast: None,
        }));

        // let t_req = reqwest::get(url_txt).await;
        // let t_req = reqwest::get("https://www.rust-lang.org").await;
        // match t_req {
        //     Ok(t) => println!("{:?}", t),
        //     Err(e) => println!("{:?}", e),
        // }
        // let tags = reqwest::get(url_txt).await.un().text().await;
        // match tags {
        //     Ok(t) => println!("{:?}", t),
        //     Err(e) => println!("{:?}", e),
        // }
        println!("lesbohovno {:?}", tags);
    }
}
