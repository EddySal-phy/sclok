use chrono::Local;
use eframe::egui;

struct ClockApp {
    decorations_visible: bool,
}

fn main() -> eframe::Result<()> 
{
    let options = eframe::NativeOptions 
    {
        viewport: egui::ViewportBuilder::default()
            .with_always_on_top()     
            .with_decorations(false)
            .with_transparent(true)         
            .with_mouse_passthrough(false)  
            .with_inner_size([150.0, 100.0]) 
            .with_position([10.0, 10.0]),   
        ..Default::default()
    };

    eframe::run_native(
        "Sclok",
        options,
        Box::new(|_cc| {
            Box::new(ClockApp {
                decorations_visible: false,
            })
        }),
    )
}


impl eframe::App for ClockApp 
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) 
    {
        let window_is_focused: bool = ctx.input(|i| i.viewport().focused.unwrap_or(false));

        if window_is_focused {
            self.decorations_visible = true;
            ctx.send_viewport_cmd(egui::ViewportCommand::Decorations(true));
        }else{
            self.decorations_visible = false;
            ctx.send_viewport_cmd(egui::ViewportCommand::Decorations(false));
        }

        // UI Layout
        let panel_frame = egui::Frame::none()
            .fill(egui::Color32::from_rgba_unmultiplied(0, 0, 0, 160))
            .rounding(8.0)
            .stroke(if self.decorations_visible {
                egui::Stroke::new(1.0, egui::Color32::GRAY)
            } else {
                egui::Stroke::NONE
            });

        egui::CentralPanel::default()
            .frame(panel_frame)
            .show(ctx, |ui| {
                let time_str = Local::now().format("%H:%M:%S").to_string();
                ui.centered_and_justified(|ui| {
                    ui.label(
                        egui::RichText::new(time_str)
                            .size(24.0)
                            .color(egui::Color32::WHITE)
                            .strong(),
                    );
                });
            });
    }
}
