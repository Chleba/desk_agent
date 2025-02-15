use std::fs;
use std::path::Path;

/// Get image files from directory for a given path
///
/// * path - Path to get image files
#[ollama_rs::function]
pub async fn get_images_from_path(
    path: String,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let img_ext = ["jpg", "jpeg", "png"];

    println!("LIST IMAGES FROM DIR");
    println!("{} - path from model", path.clone());

    let mut imgs = vec![];
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if let Some(e) = entry_path.extension() {
                if let Some(ext) = e.to_str() {
                    if img_ext.contains(&ext) {
                        if let Some(p_str) = entry_path.to_str() {
                            imgs.push(p_str.to_string());
                        }
                    }
                }
            }
        }
    }

    println!("{:?} - images", imgs);

    Ok(imgs.join("\n"))
}

/// Search image files from given path
///
/// * path - Path to search image files
#[ollama_rs::function]
pub async fn search_images_from_path(
    path: String,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let img_ext = ["jpg", "jpeg", "png"];

    println!("SEARCH IMAGES RECURSIVELY");
    println!("{} - path from model", path.clone());

    let mut imgs: Vec<String> = vec![];

    fn recurse_search(path: &Path, exts: &[&str], imgs: &mut Vec<String>) {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();

                if entry_path.is_dir() {
                    recurse_search(&entry_path, exts, imgs);
                } else if let Some(e) = entry_path.extension() {
                    if let Some(ext) = e.to_str() {
                        if exts.contains(&ext) {
                            if let Some(p_str) = entry_path.to_str() {
                                imgs.push(p_str.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    recurse_search(Path::new(&path), &img_ext, &mut imgs);

    // if let Ok(entries) = fs::read_dir(path) {
    //     for entry in entries.flatten() {
    //         let ePath = entry.path();
    //         if let Some(e) = ePath.extension() {
    //             if let Some(ext) = e.to_str() {
    //                 if img_ext.contains(&ext) {
    //                     if let Some(p_str) = ePath.to_str() {
    //                         imgs.push(p_str.to_string());
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    println!("{:?} - images", imgs);

    Ok(imgs.join("\n"))
}
