use eframe::egui::Color32;

pub enum SclokColour {
    White,
    Black,
    Orange,
    Red,
    Blue,
    Green,
    Pink,
    Yellow,
    Purple
}



pub fn set_colour(c: &SclokColour, alpha: &u8) -> Color32
{
    let colour = 
            match *c{
                SclokColour::White  => Color32::from_rgba_unmultiplied(255, 255, 255, *alpha),
                SclokColour::Black  => Color32::from_rgba_unmultiplied(0, 0, 0, *alpha),
                SclokColour::Orange => Color32::from_rgba_unmultiplied(255, 110, 97, *alpha),
                SclokColour::Red    => Color32::from_rgba_unmultiplied(218, 27, 97, *alpha),
                SclokColour::Blue   => Color32::from_rgba_unmultiplied(109, 157, 197, *alpha),
                SclokColour::Green  => Color32::from_rgba_unmultiplied(42, 157, 144, *alpha),
                SclokColour::Pink   => Color32::from_rgba_unmultiplied(253, 115, 128,*alpha),
                SclokColour::Yellow => Color32::from_rgba_unmultiplied(255, 184, 77,*alpha),
                SclokColour::Purple => Color32::from_rgba_unmultiplied(94, 75, 139,*alpha),
            };


    colour
}
