use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink};

use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use std::io::{Write, Read, stdout, stdin};

use std::io::Cursor;
use image::ImageFormat;
use image::io::Reader;
use image::{GenericImageView, Pixel};
use image::imageops::FilterType;

use reqwest::*;
use serde_json::{Result, Value};

fn display_album_cover(path: &str) {
    let img = image::open(path).unwrap();
    let scaled = img.resize(24, 24, FilterType::Nearest);
    let scaled_pixels = scaled.pixels().map(|p| p.2.to_rgb()).collect::<Vec<_>>();

    let grid = (0..24)
        .map(|i| scaled_pixels[i*24..(i+1)*24]
            .iter()
            .map(|rgb| format!("\u{1b}[48;2;{:?};{:?};{:?}m  ", rgb[0], rgb[1], rgb[2]))
            .collect::<Vec<_>>().join("")
        ).collect::<Vec<_>>().join("\u{1b}[0m\n\r");

    print!("{}\u{1b}[0m\n\r", grid);
}

fn display_album_cover_from_data(release: &str) {
    let img = image::load_from_memory(&get_cover(release)).unwrap();
    let scaled = img.resize(24, 24, FilterType::Nearest);
    let scaled_pixels = scaled.pixels().map(|p| p.2.to_rgb()).collect::<Vec<_>>();

    let grid = (0..24)
        .map(|i| scaled_pixels[i*24..(i+1)*24]
            .iter()
            .map(|rgb| format!("\u{1b}[48;2;{:?};{:?};{:?}m  ", rgb[0], rgb[1], rgb[2]))
            .collect::<Vec<_>>().join("")
        ).collect::<Vec<_>>().join("\u{1b}[0m\n\r");

    print!("{}\u{1b}[0m\n\r", grid);
}

fn toggle_needle(sink: &Sink) {
    match sink.is_paused() {
        true => sink.play(),
        false => sink.pause(),
    }
}

fn get_cover(release: &str) -> Vec<u8> {
    let request_url = format!("https://coverartarchive.org/release/{}/front", release);
    let resp = reqwest::blocking::get(&request_url).unwrap().bytes().unwrap().to_vec()
}

fn main() {
    
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let file = BufReader::new(File::open("/home/joshua/Downloads/Atomizer/1 - Jordan, Minnesota.mp3").unwrap());
    let source = Decoder::new(file).unwrap();
    sink.append(source);

    let mut needle_up = false;

    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();
    display_album_cover_from_data("76df3287-6cda-33eb-8e9a-044b5e15ffdd");

    //detecting keydown events
    for c in stdin.keys() {
        //clearing the screen and going to top left corner
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();

        display_album_cover("atomizer_cover.jpg");

        //i reckon this speaks for itself
        match c.unwrap() {
            Key::Char(' ') => toggle_needle(&sink),
            Key::Right => println!("right pressed"),
            Key::Ctrl('c') => break,
            Key::Alt('t') => println!("termion is cool"),
            _ => (),
        }

        stdout.flush().unwrap();
    }
    
}
