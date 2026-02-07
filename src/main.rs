use colored::Colorize;

const SYMBOL: char = '▚';
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
}

fn draw_all() {
    println!("All colours:");
    let test = SYMBOL.to_string();
    for x in 0..=15 {
        print!("{}", test.ansi_color(x));
    }
    println!();

    for x in 16..=231 {
        print!("{}", test.ansi_color(x));
        if (x - 16) % 36 == 35 {
            println!();
        }
    }
    for x in 232..=255 {
        print!("{}", test.ansi_color(x));
    }
    println!();
}

fn draw_rgb_reds() {
    println!("Reds with RGB:");
    let test = SYMBOL.to_string();
    for x in 0..=255 {
        print!("{}", test.truecolor(x, 0, 0));
        if x % 32 == 31 {
            println!();
        }
    }
}

fn draw_ansi_reds() {
    println!("Reds with ANSI:");
    let test = SYMBOL.to_string();
    for x in 0..=255 {
        print!("{}", test.ansi_color(map_rgb_to_ansi(x, 0, 0)));
        if x % 32 == 31 {
            println!();
        }
    }
}

fn draw_rgb_greens() {
    println!("Greens with RGB:");
    let test = SYMBOL.to_string();
    for x in 0..=255 {
        print!("{}", test.truecolor(0, x, 0));
        if x % 32 == 31 {
            println!();
        }
    }
}

fn draw_ansi_greens() {
    println!("Greens with ANSI:");
    let test = SYMBOL.to_string();
    for x in 0..=255 {
        print!("{}", test.ansi_color(map_rgb_to_ansi(0, x, 0)));
        if x % 32 == 31 {
            println!();
        }
    }
}

fn draw_rgb_blues() {
    println!("Greens with RGB:");
    let test = SYMBOL.to_string();
    for x in 0..=255 {
        print!("{}", test.truecolor(0, 0, x));
        if x % 32 == 31 {
            println!();
        }
    }
}

fn draw_ansi_blues() {
    println!("Blues with ANSI:");
    let test = SYMBOL.to_string();
    for x in 0..=255 {
        print!("{}", test.ansi_color(map_rgb_to_ansi(0, 0, x)));
        if x % 32 == 31 {
            println!();
        }
    }
}

fn draw_rgb_grey() {
    println!("Grey with RGB:");
    let test = SYMBOL.to_string();
    for x in 0..=255 {
        print!("{}", test.truecolor(x, x, x));
        if x % 32 == 31 {
            println!();
        }
    }
}

fn draw_ansi_grey() {
    println!("Grey with ANSI:");
    let test = SYMBOL.to_string();
    for x in 0..=255 {
        print!("{}", test.ansi_color(map_rgb_to_ansi(x, x, x)));
        if x % 32 == 31 {
            println!();
        }
    }
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
