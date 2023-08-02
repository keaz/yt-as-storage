use std::env;

use clap::Parser;
use env_logger::Env;
use log::info;
use yt_as_storage::{cmd::CmdArgs, io::{InputHandler, OutputHandler}};


fn main() {
    let cmd = CmdArgs::parse_from(env::args_os());
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    let mut input_handler = InputHandler::new(&cmd.file_path);
    let output_file = format!("{}/out.txt",&cmd.output_path);
    let mut output_handler = OutputHandler::new(&output_file);
    let buf_size = 255;
    let mut buf = vec![0;buf_size];
    let mut offset = 0;

    loop {
        let read_data = input_handler.read_random(offset, &mut buf);
        if read_data == 0 {
            break;
        }
        if read_data < buf_size {
            buf = buf[..read_data].to_vec();
        } 
        output_handler.write_random(offset, &buf);
        offset = offset + 256;
        buf = vec![0;buf_size];
    }
    
}
