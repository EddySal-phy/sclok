use eframe::egui;
use chrono::Local;
use super::menu::*;

pub fn draw_sclok_text(ui: &mut egui::Ui, text_font: &mut String,
                text_colour: &egui::Color32,
                use_seconds: bool)
{
    let time_str = 
        match use_seconds{
            true => Local::now().format("%H:%M:%S").to_string(),
            _    => Local::now().format("%H:%M").to_string(),
        }; 

    ui.centered_and_justified(|ui| 
    {
        // Set text style
        let sclok_text = set_text_style(ui, text_font, &time_str, &text_colour);
               
        // Set menu panel
        set_menu(sclok_text, text_font); 
    });
}

fn set_text_style(ui: &mut egui::Ui, 
                clock_font: &mut String, 
                time_str: &String,
                text_colour: &egui::Color32) -> egui::Response
{
    let text_font =
        match clock_font.as_str(){
            "Retro"     => egui::FontFamily::Name("RetroFont".into()),
            "Monospace" => egui::FontFamily::Monospace,
            _           => egui::FontFamily::Proportional,
        };
    
    let text_size =
        match clock_font.as_str(){
            "Retro"     => 90.0,
            "Monospace" => 60.0,
            _           => 70.0
        };

    let sclok_text =
        ui.label(egui::RichText::new(time_str)
                    .family(text_font)
                    .size(text_size)
                    .color(*text_colour)
                    .strong(),
        );


    sclok_text
}


// Define new fonts
pub fn set_custom_fonts(ctx: &egui::Context)
{
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "VT323_Regular".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/VT323-Regular.ttf")),
    );

    fonts.families.insert(
        egui::FontFamily::Name("RetroFont".into()),
        vec!["VT323_Regular".to_owned()],
    );

    ctx.set_fonts(fonts);
}

