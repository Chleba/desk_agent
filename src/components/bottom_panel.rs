
use super::Component;

pub struct ChatBottomPanel {}

impl ChatBottomPanel {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for ChatBottomPanel {
    fn render(&mut self, ctx: &egui::Context) {

        let ollama_img = egui::include_image!("../../assets/ollama.png");

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.set_height(100.0);
            ui.label("CENTRAL MASLO ?");

            let lamma_img = egui::Image::new(ollama_img)
                .max_width(200.0)
                .max_height(200.0)
                .tint(egui::Color32::from_rgb(10, 100, 200));
            ui.add(lamma_img);

            // let lamma_img = ollama_img.clone().image_option(ImageOptions {
            //     bg_fill: egui::Color32::from_rgb(255, 255, 0),
            // });

            // ui.add(egui::Image::new(lamma_img).bg_fill(egui::Color32::from_rgb(255, 200, 100)).max_width(100.0));

            //     // The central panel the region left after adding TopPanel's and SidePanel's
            //     // ui.heading("eframe template");

            //     // ui.horizontal(|ui| {
            //     //     ui.label("Write something: ");
            //     //     ui.text_edit_singleline(&mut self.label);
            //     // });

            //     // ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            //     // if ui.button("Increment").clicked() {
            //     //     self.value += 1.0;
            //     // }

            //     // ui.separator();

            //     // ui.add(egui::github_link_file!(
            //     //     "https://github.com/emilk/eframe_template/blob/main/",
            //     //     "Source code."
            //     // ));
        });
    }
}
