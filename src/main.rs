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
#[inline(always)]
fn to_num(n: u8) -> &'static [u8] {
    // Used to not have to ever run the integer formatter logic for u8
    unsafe {
        [
            "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15",
            "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29",
            "30", "31", "32", "33", "34", "35", "36", "37", "38", "39", "40", "41", "42", "43",
            "44", "45", "46", "47", "48", "49", "50", "51", "52", "53", "54", "55", "56", "57",
            "58", "59", "60", "61", "62", "63", "64", "65", "66", "67", "68", "69", "70", "71",
            "72", "73", "74", "75", "76", "77", "78", "79", "80", "81", "82", "83", "84", "85",
            "86", "87", "88", "89", "90", "91", "92", "93", "94", "95", "96", "97", "98", "99",
            "100", "101", "102", "103", "104", "105", "106", "107", "108", "109", "110", "111",
            "112", "113", "114", "115", "116", "117", "118", "119", "120", "121", "122", "123",
            "124", "125", "126", "127", "128", "129", "130", "131", "132", "133", "134", "135",
            "136", "137", "138", "139", "140", "141", "142", "143", "144", "145", "146", "147",
            "148", "149", "150", "151", "152", "153", "154", "155", "156", "157", "158", "159",
            "160", "161", "162", "163", "164", "165", "166", "167", "168", "169", "170", "171",
            "172", "173", "174", "175", "176", "177", "178", "179", "180", "181", "182", "183",
            "184", "185", "186", "187", "188", "189", "190", "191", "192", "193", "194", "195",
            "196", "197", "198", "199", "200", "201", "202", "203", "204", "205", "206", "207",
            "208", "209", "210", "211", "212", "213", "214", "215", "216", "217", "218", "219",
            "220", "221", "222", "223", "224", "225", "226", "227", "228", "229", "230", "231",
            "232", "233", "234", "235", "236", "237", "238", "239", "240", "241", "242", "243",
            "244", "245", "246", "247", "248", "249", "250", "251", "252", "253", "254", "255",
        ]
        .get_unchecked(n as usize)
    }
    .as_bytes()
}

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
            .map(|(x, y)| (x.0 as u32, y.0 as u32 - cli.offset_height as u32))
            .unwrap_or((50, 50));
        let (x, y) = (
            cli.width.map(|x| x as u32).unwrap_or(x),
            cli.tall.map(|x| x as u32).unwrap_or(y),
        );
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

    let width = (width as f32 * {
        if cli.multi_width >= 0.1 && cli.multi_width <= 50.0 {
            cli.multi_width
        } else {
            1.0
        }
    }) as u32;
    let image = image.into_rgba8();
    let image = resize(&image, width, height * 2, cli.filter.into());
    let mut stdout = stdout().lock();
    let mut writer = BufWriter::with_capacity(64 * 1024, stdout);
    let binding = "🮄".repeat(PIXEL_DIMENSIONS as usize);
    let pixel = binding.as_bytes();
    let placeholder = " ".repeat(PIXEL_DIMENSIONS as usize);
    let pixel_placeholder = placeholder.as_bytes();
    let mut upper_rows = image.rows().step_by(2);
    const THRESHOLD: u8 = 255 / 2;
    image.rows().skip(1).step_by(2).for_each(|row| {
        row.zip(upper_rows.next().unwrap()).for_each(|(pix, pixu)| {
            let alpha = (pixu.0[3] >= THRESHOLD, pix.0[3] >= THRESHOLD);
            match alpha {
                (true, true) => {
                    let _ = writer.write_all(b"\x1b[48;2;");
                    let _ = writer.write_all(to_num(pix.0[0]));
                    let _ = writer.write(b";");
                    let _ = writer.write_all(to_num(pix.0[1]));
                    let _ = writer.write(b";");
                    let _ = writer.write_all(to_num(pix.0[2]));
                    let _ = writer.write_all(b"m");
                    let _ = writer.write_all(b"\x1b[38;2;");
                    let _ = writer.write_all(to_num(pixu.0[0]));
                    let _ = writer.write(b";");
                    let _ = writer.write_all(to_num(pixu.0[1]));
                    let _ = writer.write(b";");
                    let _ = writer.write_all(to_num(pixu.0[2]));
                    let _ = writer.write_all(b"m");
                    let _ = writer.write_all(pixel);
                    let _ = writer.write_all(b"\x1b[39;49m");
                }
                (true, _) => {
                    let _ = writer.write_all(b"\x1b[38;2;");
                    let _ = writer.write_all(to_num(pixu.0[0]));
                    let _ = writer.write(b";");
                    let _ = writer.write_all(to_num(pixu.0[1]));
                    let _ = writer.write(b";");
                    let _ = writer.write_all(to_num(pixu.0[2]));
                    let _ = writer.write_all(b"m");
                    let _ = writer.write_all(pixel);
                    let _ = writer.write_all(b"\x1b[39;49m");
                }
                (_, true) => {
                    let _ = writer.write_all(b"\x1b[38;2;");
                    let _ = writer.write_all(to_num(pix.0[0]));
                    let _ = writer.write(b";");
                    let _ = writer.write_all(to_num(pix.0[1]));
                    let _ = writer.write(b";");
                    let _ = writer.write_all(to_num(pix.0[2]));
                    let _ = writer.write_all(b"m");
                    let _ = writer.write_all(b"\xF0\x9F\xAC\xAD\x1b[39;49m");
                }
                _ => {
                    let _ = writer.write_all(pixel_placeholder);
                }
            }
        });
        let _ = writer.write_all(b"\n");
    });
}
