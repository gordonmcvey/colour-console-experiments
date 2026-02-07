use colored::Colorize;

const SYMBOL: char = '▚';

fn main() {
    draw_all();
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
