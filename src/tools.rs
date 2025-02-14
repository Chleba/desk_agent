use std::path::Path;

use ollama_rs::coordinator::Coordinator;
use serde::de::value::Error;
use tokio::fs;

/// Get image files from directory for a given path
///
/// * path - Path to get image files
#[ollama_rs::function]
pub async fn get_images_from_path(
    path: String,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    // Ok("MASLO".to_string())

    let img_ext = ["jpg", "jpeg", "png"];

    // Ok(vec![]);

    let items = fs::read_dir(path).await;
    match items {
        Ok(files) => {
            let imgs: Vec<String> = vec![];
            for i in imgs {}
            return Ok(imgs);
        }
        Err(e) => {
            return Err("No files was found.".to_string());
        }
    }

    for item in fs::read_dir(path)? {
        let item = item?;
        let path = item.path();
        // if file_path.get_images_from_path
    }
}
