mod cli;

use std::io::{stdout, BufWriter, Write};

use cli::{Cli, Parser};
use image::{
    imageops::{resize, FilterType},
    open, GenericImageView,
};
use owo_colors::{
    DynColor, DynColors, OwoColorize,
    Stream::{Stderr, Stdout},
    Style,
};
use terminal_size::terminal_size;
const PIXEL_DIMENSIONS: u32 = 1;
const ADJUSTMENT: u32 = 2;

fn main() {
    let cli = Cli::parse();
    let image = match open(&cli.image) {
        Ok(image) => image,
        Err(err) => {
            eprintln!(
                "Encountered error: {}",
                err.if_supports_color(Stderr, |x| {
                    let style = Style::new().bold().bright_red();
                    x.style(style)
                })
            );
            std::process::exit(1)
        }
    };
    let (w, h, i_aspect) = {
        let (w, h) = image.dimensions();
        (w, h, w as f32 / h as f32)
    };
    let (width, height) = {
        let (x, y) = terminal_size()
            .map(|(x, y)| (x.0 as u32, y.0 as u32))
            .unwrap_or((50, 50));
        let x = x / PIXEL_DIMENSIONS;
        let terminal_aspect = x as f32 / y as f32;
        if terminal_aspect < i_aspect {
            // If the terminal is wider than the image
            (
                x * ADJUSTMENT,
                (y as f32 / (i_aspect / terminal_aspect)) as u32,
            )
        } else {
            (
                (x as f32 / (terminal_aspect / i_aspect)) as u32 * ADJUSTMENT,
                y,
            )
        }
    };
    let image = image.into_rgba8();
    let image = resize(&image, width, height * 2, cli.filter.into());
    let mut stdout = stdout().lock();
    let mut writer = BufWriter::new(stdout);
    let binding = "🮄".repeat(PIXEL_DIMENSIONS as usize);
    let pixel = binding.as_str();
    let placeholder = " ".repeat(PIXEL_DIMENSIONS as usize);
    let pixel_placeholder = placeholder.as_bytes();
    let mut upper_rows = image.rows().step_by(2);
    image.rows().skip(1).step_by(2).for_each(|row| {
        row.zip(upper_rows.next().unwrap()).for_each(|(pix, pixu)| {
            if pix.0[3] > 255 / 2 {
                writer.write_fmt(format_args!(
                    "{}",
                    pixel
                        .on_color(DynColors::Rgb(pix.0[0], pix.0[1], pix.0[2]))
                        .color(DynColors::Rgb(pixu.0[0], pixu.0[1], pixu.0[2]))
                ));
            } else {
                writer.write_all(pixel_placeholder);
            }
        });
        writer.write(b"\n");
    });
}
