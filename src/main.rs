pub mod cmd;
pub mod io;
pub mod utils;

use indicatif::ProgressBar;
use io::{InputHandler, OutputHandler};
use log::info;
use std::process::exit;
use std::{fs, fs::OpenOptions, io::prelude::*, process::Command};

fn main() {
    io::create_dirs(vec!["vid2fps", "vidout", "vidin", "textout", "textin"]);

    loop {
        let mut line = String::new();
        println!("Enter encode or decode (e/d) of the input file? [q for quit]:");
        std::io::stdin().read_line(&mut line).unwrap();

        if line.trim_end().to_string() == "e" {
            create_video_out();
        } else if line.trim_end().to_string() == "d" {
            read_video_in();
        } else if line.trim_end().to_string() == "q" {
            exit(0);
        }
    }
}

pub fn create_video_out() {
    let mut input_handler = InputHandler::new(&io::get_first_in_dir(fs::read_dir("textin")));

    let res_bit = (256 * 144) / 8 as u64;
    let buf_size = res_bit as usize;
    let mut buf = vec![0; buf_size];
    let mut offset = 0 as u64;
    let mut img_index = 1;

    io::clear_vid2fps();

    info!("Generating frames: ");
    let progress_bar = ProgressBar::new(input_handler.get_file_size() / res_bit);

    loop {
        let read_data = input_handler.read_input_data(offset, &mut buf);
        if read_data == 0 {
            break;
        }
        if read_data < buf_size {
            buf = buf[..read_data].to_vec();
        }

        OutputHandler::encode_frames(&buf, format!("{:04}", img_index).as_str());
        offset = offset + res_bit + 1;
        buf = vec![0; buf_size];
        img_index += 1;
        progress_bar.inc(1);
    }
    progress_bar.finish_with_message("Frame generation done");

    info!("Combining frames");

    let progress_spinner = utils::progress();

    io::clear_vidout();
    Command::new("powershell")
        .args(&[
            "/C",
            "ffmpeg -framerate 1 -i vid2fps/output%04d.png -r 30 vidout/video.mp4",
        ])
        .output()
        .expect("failed to execute process");

    io::clear_vid2fps();

    progress_spinner.finish_with_message("Successfully created the video.\n");
}

pub fn read_video_in() {
    info!("Reading the frames from the video");

    let progress_spinner = utils::progress();

    let mut img_index = 0;

    io::clear_vid2fps();

    let ffmpeg_cmd = &format!(
        "ffmpeg -i '{}' -vf fps=1 vid2fps/extracted%04d.png",
        io::get_first_in_dir(fs::read_dir("vidin"))
    );

    Command::new("powershell")
        .args(&["/C", &ffmpeg_cmd])
        .output()
        .unwrap();

    let frame_count = fs::read_dir("vid2fps").unwrap().count();

    fs::remove_file("textout/decoded_video.txt").ok();
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("textout/decoded_video.txt")
        .unwrap();

    loop {
        img_index += 1;

        let data = OutputHandler::decode_frames(format!("{:04}", img_index).as_str());

        if let Err(e) = writeln!(file, "{}", data.as_str()) {
            eprintln!("Couldn't write to file: {}", e);
        }

        if img_index == frame_count {
            break;
        }
    }

    progress_spinner.finish_with_message("Successfully decoded the video.\n");
}
