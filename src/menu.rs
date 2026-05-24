use eframe::egui;

pub fn set_menu(sclok_text: egui::Response, clock_font: &mut String)
{
    sclok_text.context_menu(|ui|
    {
        if ui.selectable_value(clock_font,
                            "Retro".to_string(),
                            "Retro Font")
            .changed() {
            ui.close_menu();
        }
        
        if ui.selectable_value(clock_font,
                            "Default".to_string(),
                            "Simple Font")
            .changed() {
            ui.close_menu();
        }

        if ui.selectable_value(clock_font,
                            "Monospace".to_string(),
                            "Terminal Font")
            .changed() {
            ui.close_menu();
        }
    });
}
