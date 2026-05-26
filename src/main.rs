use eframe::egui;

mod colour;
mod parse;
mod menu;
mod text;
mod panel_frame;

use colour::*;
use parse::*;
use text::*;
use panel_frame::*;

struct Sclok {
    use_seconds: bool,
    static_font: bool,
    text_colour: SclokColour,
    bg_colour: SclokColour,
    bg_alpha: u8, 
    decorations_visible: bool,
    text_font: String,
    corner_radius: egui::Rounding,
}

fn main() -> eframe::Result<()> 
{
    let (use_seconds, static_font, text_colour, bg_colour, bg_alpha) = parse_args();
    
    let sclok = Sclok{
        use_seconds,
        static_font,
        text_colour, 
        bg_colour,
        bg_alpha,
        decorations_visible: false,
        text_font: "Monospace".to_string(), //default font
        corner_radius: 18.0.into(), // default UI rouding
    };

    let options = eframe::NativeOptions 
    {
        viewport: egui::ViewportBuilder::default()
            .with_always_on_top()     
            .with_decorations(false)
            .with_transparent(true)         
            .with_mouse_passthrough(false)  
            .with_inner_size(match use_seconds{
                                true  => [300.0, 110.0],
                                false => [190.0, 110.0],
                            }) 
            .with_position([10.0, 10.0]),   
        ..Default::default()
    };
    
    eframe::run_native(
        "sclok",
        options,
        Box::new(|_cc| {
            set_custom_fonts(&_cc.egui_ctx);
            Box::new(sclok)
        }),
    )
}


impl eframe::App for Sclok 
{
    // Force the UI window to be fully transparent
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) 
    {
        let window_is_focused: bool = ctx.input(|i| i.viewport().focused.unwrap_or(false));
        let use_seconds   = self.use_seconds;
        let static_font   = self.static_font;
        let corner_radius = self.corner_radius;
        let text_colour   = set_colour(&self.text_colour, &255); //always use bright text
        let bg_alpha      = self.bg_alpha;
        let bg_colour     = set_colour(&self.bg_colour, &bg_alpha); 


        if window_is_focused {
            self.decorations_visible = true;
            ctx.send_viewport_cmd(egui::ViewportCommand::Decorations(true));
        }else{
            self.decorations_visible = false;
            ctx.send_viewport_cmd(egui::ViewportCommand::Decorations(false));
        }

        // UI layout
        let panel_frame = 
            set_panel_frame(bg_colour,
                            corner_radius.into(),
                            self.decorations_visible);
        
        // Draw text on panel
        egui::CentralPanel::default()
            .frame(panel_frame)
            .show(ctx, |ui| {
                draw_sclok_text(ui, ctx, &mut self.text_font, &text_colour, use_seconds, static_font);   
            });
    }
}









