use eframe::egui;


pub fn set_panel_frame(bg_colour: egui::Color32, 
                corner_radius: egui::Rounding,
                decorations_visible: bool) -> eframe::egui::Frame
{
    let panel_frame =
        egui::Frame::none()
        .fill(bg_colour)
        .rounding(corner_radius)
        .stroke(if decorations_visible {
            egui::Stroke::new(1.0, egui::Color32::GRAY)
        } else {
            egui::Stroke::NONE
        });


    panel_frame
}

