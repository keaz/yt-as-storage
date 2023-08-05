pub mod io;
pub mod cmd;

use cmd::CmdArgs;
use io::{InputHandler, OutputHandler};
use std::{env, fs, fs::{OpenOptions}, io::{prelude::*}, process::{Command}, time::Duration};
use std::process::exit;
use clap::Parser;
use env_logger::Env;
use log::info;
use indicatif::{ProgressBar, ProgressStyle};

fn main() {

    io::create_dirs();

    loop {
        let cmd = CmdArgs::parse_from(env::args_os());

        let mut line = String::new();
        println!("Enter encode or decode (e/d) of the input file? [q for quit]:");
        std::io::stdin().read_line(&mut line).unwrap();

        if line.trim_end().to_string() == "e" {
            create_video_out(cmd);
        } else if line.trim_end().to_string() == "d" {
            read_video_in();
        } else if line.trim_end().to_string() == "q" {
            exit(0);
        }
    }

}

pub fn create_video_out(cmd: CmdArgs) {

    let prog = progress();

    let mut input_handler = InputHandler::new(&cmd.file_path);

    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let res_bit = (256*144)/8 as u64;
    let buf_size = res_bit as usize;
    let mut buf = vec![0;buf_size];
    let mut offset = 0 as u64;
    let mut img_index = 1;

    io::clear_vid2fps();

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
        buf = vec![0;buf_size];
        img_index += 1;
    }

    io::clear_vidout();

    info!("Combining frames");
    Command::new("cmd")
        .args(&["/C", "ffmpeg -framerate 1 -i vid2fps/output%04d.png -r 30 vidout/video.mp4"]).output()
        .expect("failed to execute process");

    io::clear_vid2fps();

    prog.finish_with_message("Successfully created the video.\n");
}

pub fn read_video_in() {
    let prog = progress();

    let mut img_index = 0;

    io::clear_vid2fps();

    info!("Extracting the video to frames");
    Command::new("cmd")
        .args(&["/C", "ffmpeg -i vidout/video.mp4 -vf fps=1 vid2fps/extracted%04d.png"]).output()
        .expect("failed to execute process");

    let frame_count = fs::read_dir("vid2fps").unwrap().count();

    fs::remove_file("out.txt").ok();
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("out.txt")
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

    prog.finish_with_message("Successfully decoded the video.\n");
}

fn progress() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.red} {msg}")
            .unwrap()
            .tick_strings(&[
                "  ▶⛔       ◀ ",
                " ▶  ⛔      ◀ ",
                " ▶   ⛔     ◀ ",
                " ▶    ⛔    ◀ ",
                " ▶     ⛔   ◀ ",
                " ▶      ⛔  ◀ ",
                " ▶       ⛔◀  ",
                " ▶      ⛔  ◀ ",
                " ▶     ⛔   ◀ ",
                " ▶    ⛔    ◀ ",
                " ▶   ⛔     ◀ ",
                " ▶  ⛔      ◀ "
            ]),
    );
    pb.set_message("Processing...");

    pb
}