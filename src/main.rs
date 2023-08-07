pub mod cmd;
pub mod io;
pub mod utils;

use clap::Parser;
use env_logger::Env;

use indicatif::ProgressBar;
use io::{InputHandler, OutputHandler};
use log::{info, debug};

use std::env;
use std::process::exit;

use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::{fs, fs::OpenOptions, io::prelude::*, process::Command};

use crate::io::Data;

fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    io::create_dirs(vec!["vid2fps", "vidout", "vidin", "textout", "textin"]);

    let cmd = cmd::CmdArgs::parse_from(env::args_os());

    loop {
        let mut line = String::new();
        println!("Enter encode or decode (e/d) of the input file? [q for quit]:");
        std::io::stdin().read_line(&mut line).unwrap();

        if line.trim_end().to_string() == "e" {
            create_video_out(&&cmd.file_path);
        } else if line.trim_end().to_string() == "d" {
            read_video_in();
        } else if line.trim_end().to_string() == "q" {
            exit(0);
        }
    }
}

pub fn create_video_out(input_file: &String) {
    let mut input_handler = InputHandler::new(&input_file);

    let res_bit = (256 * 144) / 8 as u64;
    let buf_size = res_bit as usize;

    io::clear_vid2fps();

    let progress_bar = Arc::new(ProgressBar::new(input_handler.get_file_size() / res_bit));
    let total_frames = input_handler.get_file_size() / res_bit;
    info!("Generating frames: {}", total_frames);
  
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build()
        .unwrap();
    let (sender, receiver) = channel();
    for img_index in 1..total_frames + 1 {
        let mut input_handler = input_handler.clone();

        let sender = sender.clone();
        pool.spawn(move || {
            let mut buf = vec![0; buf_size];
            let offset = (img_index - 1) * buf_size as u64;
            let read_data = input_handler.read_input_data(offset, &mut buf);
            if read_data == 0 {
                return;
            }
            if read_data < buf_size {
                buf = buf[..read_data].to_vec();
            }

            let data = Data::new(img_index, buf);
            sender.send(data).unwrap();
        });
    }

    for data in receiver {
        let progress_bar = progress_bar.clone();
        pool.spawn(move || {
            OutputHandler::encode_frames(&data.buf, format!("{:04}", data.index).as_str());
            debug!("Encoding image {}", data.index);
            progress_bar.inc(1);
        });
    }

    progress_bar.finish_with_message("Frame generation done");

    info!("Combining frames");

    // let progress_spinner = utils::progress();

    io::clear_vidout();
    execute_video_out_ffmped();

    io::clear_vid2fps();

    // progress_spinner.finish_with_message("Successfully created the video.\n");
}

#[cfg(target_os = "windows")]
fn execute_video_out_ffmped() {
    Command::new("powershell")
        .args(&[
            "/C",
            "ffmpeg -framerate 1 -i vid2fps/output%04d.png -r 30 vidout/video.mp4",
        ])
        .output()
        .expect("failed to execute process");
}

#[cfg(target_os = "linux")]
fn execute_video_out_ffmped() {
    Command::new("sh")
        .args(&[
            "-c",
            "ffmpeg -framerate 1 -i vid2fps/output%04d.png -r 30 vidout/video.mp4",
        ])
        .output()
        .unwrap();
}

pub fn read_video_in() {
    info!("Reading the frames from the video");

    let progress_spinner = utils::progress();

    let mut img_index = 0;

    io::clear_vid2fps();

    execute_ffmpeg();

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

#[cfg(target_os = "windows")]
fn execute_ffmpeg() {
    let ffmpeg_cmd = &format!(
        "ffmpeg -i '{}' -vf fps=1 vid2fps/extracted%04d.png",
        io::get_first_in_dir(fs::read_dir("vidin"))
    );

    Command::new("powershell")
        .args(&["/C", &ffmpeg_cmd])
        .output()
        .unwrap();
}

#[cfg(target_os = "linux")]
fn execute_ffmpeg() {
    let ffmpeg_cmd = &format!(
        "ffmpeg -i '{}' -vf fps=1 vid2fps/extracted%04d.png",
        io::get_first_in_dir(fs::read_dir("vidin"))
    );

    Command::new("sh")
        .args(&["-c", &ffmpeg_cmd])
        .output()
        .unwrap();
}
