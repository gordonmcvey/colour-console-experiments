mod rgb_colour;

use colored::Colorize;
use crate::rgb_colour::RgbColour;

const SYMBOL: &str = "█";
const CLUT_START: u8 = 16;
const GREYSCALE_START: u8 = 232;
const RED_INTERVAL: u8 = 36;
const GREEN_INTERVAL: u8 = 6;
const SCALING_FACTOR: u32 = 1000;
// There are 6 brightnesses for each colour, 256 / 6 = 43.666 recurring
const COLOUR_STEP: u32 = 43667;
// There are 24 greyscales, 256 / 24 = 10.666 recurring
const GREYSCALE_STEP: u32 = 10667;

fn main() {
    draw_all();
    draw_rgb_reds();
    draw_ansi_reds();
    draw_rgb_greens();
    draw_ansi_greens();
    draw_rgb_blues();
    draw_ansi_blues();
    draw_rgb_grey();
    draw_ansi_grey();
    draw_ansi_magenta();
    draw_dithered_ansi_magenta();

    println!("Gradient:");
    for colour in gradient(RgbColour::new(255, 0, 0), RgbColour::new(0, 0, 255), 160) {
        let index = map_rgb_to_ansi(colour.red, colour.green, colour.blue);
        print!("{}", SYMBOL.ansi_color(index));
    }
    println!();

    println!("Spectrum:");
    for colour in gradient(RgbColour::new(255, 0, 0), RgbColour::new(255, 127, 0), 8) {
        let index = map_rgb_to_ansi(colour.red, colour.green, colour.blue);
        print!("{}", SYMBOL.ansi_color(index));
    }
    for colour in gradient(RgbColour::new(255, 143, 0), RgbColour::new(255, 255, 0), 8) {
        let index = map_rgb_to_ansi(colour.red, colour.green, colour.blue);
        print!("{}", SYMBOL.ansi_color(index));
    }
    for colour in gradient(RgbColour::new(239, 255, 0), RgbColour::new(127, 255, 0), 8) {
        let index = map_rgb_to_ansi(colour.red, colour.green, colour.blue);
        print!("{}", SYMBOL.ansi_color(index));
    }
    for colour in gradient(RgbColour::new(111, 255, 0), RgbColour::new(0, 255, 0), 8) {
        let index = map_rgb_to_ansi(colour.red, colour.green, colour.blue);
        print!("{}", SYMBOL.ansi_color(index));
    }
    for colour in gradient(RgbColour::new(0, 255, 16), RgbColour::new(0, 255, 127), 8) {
        let index = map_rgb_to_ansi(colour.red, colour.green, colour.blue);
        print!("{}", SYMBOL.ansi_color(index));
    }
    for colour in gradient(RgbColour::new(0, 255, 143), RgbColour::new(0, 255, 255), 8) {
        let index = map_rgb_to_ansi(colour.red, colour.green, colour.blue);
        print!("{}", SYMBOL.ansi_color(index));
    }
    for colour in gradient(RgbColour::new(0, 239, 255), RgbColour::new(0, 127, 255), 8) {
        let index = map_rgb_to_ansi(colour.red, colour.green, colour.blue);
        print!("{}", SYMBOL.ansi_color(index));
    }
    for colour in gradient(RgbColour::new(0, 111, 255), RgbColour::new(0, 0, 255), 8) {
        let index = map_rgb_to_ansi(colour.red, colour.green, colour.blue);
        print!("{}", SYMBOL.ansi_color(index));
    }
    for colour in gradient(RgbColour::new(16, 0, 255), RgbColour::new(127, 0, 255), 8) {
        let index = map_rgb_to_ansi(colour.red, colour.green, colour.blue);
        print!("{}", SYMBOL.ansi_color(index));
    }
    for colour in gradient(RgbColour::new(143, 0, 255), RgbColour::new(255, 0, 255), 8) {
        let index = map_rgb_to_ansi(colour.red, colour.green, colour.blue);
        print!("{}", SYMBOL.ansi_color(index));
    }
    for colour in gradient(RgbColour::new(255, 0, 239), RgbColour::new(255, 0, 0), 8) {
        let index = map_rgb_to_ansi(colour.red, colour.green, colour.blue);
        print!("{}", SYMBOL.ansi_color(index));
    }
    println!();
}

fn draw_all() {
    println!("All colours:");
    for x in 0..=15 {
        print!("{}", SYMBOL.ansi_color(x));
    }
    println!();

    for x in 16..=231 {
        print!("{}", SYMBOL.ansi_color(x));
        if (x - 16) % 36 == 35 {
            println!();
        }
    }
    for x in 232..=255 {
        print!("{}", SYMBOL.ansi_color(x));
    }
    println!();
}

fn draw_rgb_reds() {
    println!("Reds with RGB:");
    for x in 0..=255 {
        print!("{}", SYMBOL.truecolor(x, 0, 0));
        if x % 32 == 31 {
            println!();
        }
    }
}

fn draw_ansi_reds() {
    println!("Reds with ANSI:");
    for x in 0..=255 {
        print!("{}", SYMBOL.ansi_color(map_rgb_to_ansi(x, 0, 0)));
        if x % 32 == 31 {
            println!();
        }
    }
}

fn draw_rgb_greens() {
    println!("Greens with RGB:");
    for x in 0..=255 {
        print!("{}", SYMBOL.truecolor(0, x, 0));
        if x % 32 == 31 {
            println!();
        }
    }
}

fn draw_ansi_greens() {
    println!("Greens with ANSI:");
    for x in 0..=255 {
        print!("{}", SYMBOL.ansi_color(map_rgb_to_ansi(0, x, 0)));
        if x % 32 == 31 {
            println!();
        }
    }
}

fn draw_rgb_blues() {
    println!("Blues with RGB:");
    for x in 0..=255 {
        print!("{}", SYMBOL.truecolor(0, 0, x));
        if x % 32 == 31 {
            println!();
        }
    }
}

fn draw_ansi_blues() {
    println!("Blues with ANSI:");
    for x in 0..=255 {
        print!("{}", SYMBOL.ansi_color(map_rgb_to_ansi(0, 0, x)));
        if x % 32 == 31 {
            println!();
        }
    }
}

fn draw_rgb_grey() {
    println!("Grey with RGB:");
    for x in 0..=255 {
        print!("{}", SYMBOL.truecolor(x, x, x));
        if x % 32 == 31 {
            println!();
        }
    }
}

fn draw_ansi_grey() {
    println!("Grey with ANSI:");
    for x in 0..=255 {
        print!("{}", SYMBOL.ansi_color(map_rgb_to_ansi(x, x, x)));
        if x % 32 == 31 {
            println!();
        }
    }
}

fn draw_ansi_magenta() {
    println!("Magenta with ANSI:");
    for x in 0..=255 {
        print!("{}", SYMBOL.ansi_color(map_rgb_to_ansi(x, 0, x)));
        if x % 32 == 31 {
            println!();
        }
    }
}

fn draw_dithered_ansi_magenta() {
    println!("Dithered magenta with ANSI:");
    let indexes = find_indexes_magenta();
    let dithers: [&str; 5] = [" ", "░", "▒", "▓", "█"];

    for x in 0..indexes.len() - 1 {
        for symbol in dithers.iter() {
            print!("{}", symbol.ansi_color(indexes[x + 1]).on_ansi_color(indexes[x]));
        }
    }
    println!();
    //
    // let mut ansi_index: u8;
    // for x in 0..=255 {
    //     ansi_index = map_rgb_to_ansi(x, 0, x);
    //     print!("{}", dithers[(x % 5) as usize].ansi_color(ansi_index + 1).on_ansi_color(ansi_index));
    //     if x % 32 == 31 {
    //         println!();
    //     }
    // }
}

fn map_rgb_to_ansi(r: u8, g: u8, b: u8) -> u8 {
    /*
     * If the RGB values indicate a grey, then map to one of the 24 greyscales at the end of the
     * CLUT (they provide better results than the 6 greyscales in the 216 colour block)
     *
     * @TODO: trigger if the RGB values are within a delta instead of an exact match
     */
    if r == g && g == b {
        return ((r as u32 * SCALING_FACTOR) / GREYSCALE_STEP) as u8 + GREYSCALE_START;
    }

    let scaled_r = (r as u32 * SCALING_FACTOR) / COLOUR_STEP;
    let scaled_g = (g as u32 * SCALING_FACTOR) / COLOUR_STEP;
    let scaled_b = (b as u32 * SCALING_FACTOR) / COLOUR_STEP;

    CLUT_START
        + (RED_INTERVAL * scaled_r as u8)
        + (GREEN_INTERVAL * scaled_g as u8)
        + scaled_b as u8
}

fn find_boundaries_magenta() -> Vec<u8> {
    let mut boundaries: Vec<u8> = Vec::new();
    let mut last_index: Option<u8> = None;
    let mut current_index: u8;

    for x in 0..=255 {
        current_index = map_rgb_to_ansi(x, 0, x);

        if GREYSCALE_START == current_index {
            continue;
        }

        if current_index != last_index.unwrap_or_default() {
            last_index = Some(current_index);
            boundaries.push(x);
        }
    }

    boundaries
}

fn find_indexes_magenta() -> Vec<u8> {
    let mut indexes: Vec<u8> = Vec::new();
    let mut last_index: Option<u8> = None;
    let mut current_index: u8;

    for x in 0..=255 {
        current_index = map_rgb_to_ansi(x, 0, x);

        if GREYSCALE_START == current_index {
            continue;
        }

        if current_index != last_index.unwrap_or_default() {
            last_index = Some(current_index);
            indexes.push(current_index);
        }
    }

    indexes
}


fn gradient(start: RgbColour, end: RgbColour, steps: u32) -> Vec<RgbColour> {
    (0..=steps)
        .map(|step| {
            let ratio = step as f64 / steps as f64;
            RgbColour::new(
                RgbColour::lerp(start.red, end.red, ratio),
                RgbColour::lerp(start.green, end.green, ratio),
                RgbColour::lerp(start.blue, end.blue, ratio),
            )
        })
        .collect()
    // let mut colours: Vec<RgbColour> = Vec::new();
    // let mut ratio: i32;
    // let mut new_red: u8;
    // let mut new_green: u8;
    // let mut new_blue: u8;
    //
    // let start_red_i32 = start.red as i32;
    // let start_green_i32 = start.green as i32;
    // let start_blue_i32 = start.blue as i32;
    // let end_red_i32 = end.red as i32;
    // let end_green_i32 = end.green as i32;
    // let end_blue_i32 = end.blue as i32;
    //
    // for step in 0..=steps {
    //     ratio = ((step as f64 / steps as f64) * SCALING_FACTOR as f64) as i32;
    //     new_red = (start_red_i32 + (ratio * (end_red_i32 - start_red_i32) / SCALING_FACTOR as i32)) as u8;
    //     new_green = (start_green_i32 + (ratio * (end_green_i32 - start_green_i32) / SCALING_FACTOR as i32)) as u8;
    //     new_blue = (start_blue_i32 + (ratio * (end_blue_i32 - start_blue_i32) / SCALING_FACTOR as i32)) as u8;
    //
    //     colours.push(RgbColour::new(new_red, new_green, new_blue));
    // }
    //
    // colours
}
