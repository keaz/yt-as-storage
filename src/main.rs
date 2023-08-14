pub mod cmd;
pub mod io;
pub mod utils;

use clap::Parser;
use env_logger::Env;

use indicatif::ProgressBar;
use io::{InputHandler, OutputHandler};
use log::{debug, info};

use std::env;

use crossbeam_utils::sync::WaitGroup;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::{fs, fs::OpenOptions, io::prelude::*, process::Command};

use crate::io::Data;

fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let cmd = cmd::CmdArgs::parse_from(env::args_os());

    let vid2fps = format!("{}/vid2fps", &cmd.output_path);
    let vidout = format!("{}/vidout", &cmd.output_path);
    let vidin = format!("{}/vidin", &cmd.output_path);
    let textout = format!("{}/textout", &cmd.output_path);

    io::create_dirs(vec![vid2fps, vidout, vidin, textout]);

    match cmd.mode {
        cmd::Mode::Encode => {
            create_video_out(&cmd.file_path, &cmd.output_path);
        }
        cmd::Mode::Decode => {
            read_video_in(&cmd.file_path, &cmd.output_path);
        }
    }
}

pub fn create_video_out(input_file: &String, output_folder: &String) {
    debug!("Started to create viode out for file {}", input_file);
    let mut input_handler = InputHandler::new(&input_file);

    let res_bit = (256 * 144) / 8 as u64;
    let buf_size = res_bit as usize;

    io::clear_vid2fps(output_folder);

    let total_frames = (input_handler.get_file_size() / res_bit) + 1;
    let progress_bar = Arc::new(ProgressBar::new(total_frames));
    info!("Generating {} frames", total_frames);

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build()
        .unwrap();
    let (sender, receiver) = channel();

    let wg = WaitGroup::new();

    for img_index in 1..total_frames + 1 {
        let mut input_handler = input_handler.clone();
        let wg = wg.clone();

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
            drop(wg);
        });
    }

    wg.wait();

    let wg = WaitGroup::new();
    let mut index = 0;
    for data in receiver {
        let output_folder = output_folder.clone();
        let progress_bar = progress_bar.clone();
        let wg = wg.clone();
        pool.spawn(move || {
            OutputHandler::encode_frames(
                &data.buf,
                format!("{:04}", data.index).as_str(),
                &output_folder,
            );
            debug!("Encoding image {}", data.index);
            progress_bar.inc(1);
            drop(wg);
        });
        index += 1;
        if index == total_frames {
            break;
        }
    }

    wg.wait();
    progress_bar.finish_with_message("Frame generation done");

    info!("Combining frames");

    let progress_spinner = utils::progress();

    io::clear_vidout(output_folder);
    execute_video_out_ffmped(output_folder);

    io::clear_vid2fps(output_folder);

    progress_spinner.finish_with_message("Successfully created the video.\n");
}

#[cfg(target_family = "windows")]
fn execute_video_out_ffmped(output_path: &String) {
    let cmd = format!(
        "ffmpeg -framerate 1 -i {}/vid2fps/output%04d.png -r 30 {}/vidout/video.mp4",
        output_path, output_path
    );
    Command::new("powershell")
        .args(&["/C", &cmd])
        .output()
        .expect("failed to execute process");
}

#[cfg(target_family = "unix")]
fn execute_video_out_ffmped(output_path: &String) {
    let cmd = format!(
        "ffmpeg -framerate 1 -i {}/vid2fps/output%04d.png -r 30 {}/vidout/video.mp4",
        output_path, output_path
    );
    Command::new("sh").args(&["-c", &cmd]).output().unwrap();
}

pub fn read_video_in(input_file: &String, output_folder: &String) {
    info!("Reading the frames from the video");

    let progress_spinner = utils::progress();

    let mut img_index = 0;

    io::clear_vid2fps(output_folder);

    execute_ffmpeg(input_file, output_folder);
    let frame_folder = format!("{}/vid2fps", output_folder);
    let frame_count = fs::read_dir(frame_folder).unwrap().count();
    let out_txt = format!("{}/textout/decoded_video.txt", output_folder);
    fs::remove_file(&out_txt).ok();
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&out_txt)
        .unwrap();

    loop {
        img_index += 1;

        let data =
            OutputHandler::decode_frames(format!("{:04}", img_index).as_str(), output_folder);

        file.write(data.as_bytes()).expect("Unable to write data");

        if img_index == frame_count {
            break;
        }
    }

    progress_spinner.finish_with_message("Successfully decoded the video.\n");
}

#[cfg(target_family = "windows")]
fn execute_ffmpeg(input_file: &String, output_folder: &String) {
    let ffmpeg_cmd = &format!(
        "ffmpeg -i '{}' -vf fps=1 {}/vid2fps/extracted%04d.png",
        input_file, output_folder
    );

    Command::new("powershell")
        .args(&["/C", &ffmpeg_cmd])
        .output()
        .unwrap();
}

#[cfg(target_family = "unix")]
fn execute_ffmpeg(input_file: &String, output_folder: &String) {
    let ffmpeg_cmd = &format!(
        "ffmpeg -i '{}' -vf fps=1 {}/vid2fps/extracted%04d.png",
        input_file, output_folder
    );

    Command::new("sh")
        .args(&["-c", &ffmpeg_cmd])
        .output()
        .unwrap();
}
