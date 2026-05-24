use std::env;
use super::colour::*;

pub fn parse_args() -> (bool, SclokColour, SclokColour, u8)
{
    // Default values 
    let mut use_seconds = false;
    let mut text_colour = SclokColour::White;
    let mut bg_colour = SclokColour::Black;
    let mut bg_alpha: u8 = 255;

    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() 
    {
        match arg.as_str() {
            "-S" => use_seconds = true,
            "-c" => {
                if let Some(val) = args.next() {
                    text_colour = match val.as_str() {
                        "w"  | "white"  => SclokColour::White,
                        "bk" | "black"  => SclokColour::Black,
                        "o"  | "orange" => SclokColour::Orange,
                        "r"  | "red"    => SclokColour::Red,
                        "b"  | "blue"   => SclokColour::Blue,
                        "g"  | "green"  => SclokColour::Green,
                        "pk" | "pink"   => SclokColour::Pink,
                        "y"  | "yellow" => SclokColour::Yellow,
                        "p"  | "purple" => SclokColour::Purple,
                        _ => {
                            eprintln!("Warning: unknown text color '{}', defaulting to White", val);
                            SclokColour::White
                        }
                    };
                }
            }
            "-b" => {
                if let Some(val) = args.next() {
                    bg_colour = match val.as_str() {
                        "w"  | "white"  => SclokColour::White,
                        "bk" | "black"  => SclokColour::Black,
                        "o"  | "orange" => SclokColour::Orange,
                        "r"  | "red"    => SclokColour::Red,
                        "b"  | "blue"   => SclokColour::Blue,
                        "g"  | "green"  => SclokColour::Green,
                        "pk" | "pink"   => SclokColour::Pink,
                        "y"  | "yellow" => SclokColour::Yellow,
                        "p"  | "purple" => SclokColour::Purple,
                        _ => {
                            eprintln!("Warning: unknown background color '{}', defaulting to Black", val);
                            SclokColour::Black
                        }
                    };
                }
            }
            "-t" => {
                bg_alpha = 160;
                if let Some(val) = args.next() {
                    bg_alpha = match val.parse::<u8>() {
                        Ok(parsed_num) => parsed_num, 
                        Err(_) => {
                            eprintln!("Warning: invalid alpha value '{}', defaulting to 255", val);
                            255
                        }
                    };
                }
            }
            "-h"  => help(),
            "-lc" => list_colours(),
            _ => {
                eprintln!("Warning: unknown flag '{}' ignored.", arg);
            }
        }
    }

    (use_seconds, text_colour, bg_colour, bg_alpha)
}


#[inline(always)]
fn help() {
    println!("    -S    Display seconds");
    println!("    -c    Set text/number color");
    println!("    -b    Set background color");
    println!("    -t    Set background transparency [0-255] (default: 160)");
    println!("    -lc   List colours");
    println!("    -h    Help");
    std::process::exit(1);
}

fn list_colours() {
println!("Available colours and their abbreviations:
    -c, -b   w  | white     White
    -c, -b   bk | black     Black
    -c, -b   o  | orange    Orange
    -c, -b   r  | red       Red
    -c, -b   b  | blue      Blue
    -c, -b   g  | green     Green
    -c, -b   pk | pink      Pink
    -c, -b   y  | yellow    Yellow
    -c, -b   p  | purple    Purple");
    std::process::exit(1);
}
