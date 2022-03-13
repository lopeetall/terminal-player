use image::{GenericImageView, Pixel};
use image::imageops::FilterType;

pub fn display_album_cover(path: &str) {
    let img = image::open(path).unwrap();
    let scaled = img.resize(24, 24, FilterType::Nearest);
    let scaled_pixels = scaled.pixels().map(|p| p.2.to_rgb()).collect::<Vec<_>>();

    let grid = (0..24)
        .map(|i| scaled_pixels[i*24..(i+1)*24]
            .iter()
            .map(|rgb| format!("\u{1b}[48;2;{:?};{:?};{:?}m  ", rgb[0], rgb[1], rgb[2]))
            .collect::<Vec<_>>().join("")
        ).collect::<Vec<_>>().join("\u{1b}[0m\n");

    println!("{}\u{1b}[0m", grid);
}