use std::env;

use clap::Parser;
use env_logger::Env;
use image::{Rgb, ImageBuffer, RgbImage};
use log::{info, debug};
use yt_as_storage::{cmd::CmdArgs, io::{InputHandler, OutputHandler}};


fn main() {
    let cmd = CmdArgs::parse_from(env::args_os());
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    let mut input_handler = InputHandler::new(&cmd.file_path);
    let output_file = format!("{}/tibco",&cmd.output_path);
    let mut output_handler = OutputHandler::new(&output_file);
    let buf_size: usize = 255;
    let image_size: u32 = (buf_size +1).try_into().unwrap();
    let mut buf = vec![0;buf_size];
    let mut offset = 0;

    // let mut img: RgbImage = ImageBuffer::new(image_size,image_size);

    // let mut x = 0;
    // let mut y = 0;
    // let mut seq = 1;
    // loop {
    //     let read_data = input_handler.read_random(offset, &mut buf);
    //     if read_data == 0 {
    //         break;
    //     }
    //     if read_data < buf_size {
    //         buf = buf[..read_data].to_vec();
    //     }

    //     buf.iter().for_each(|data| {
    //         debug!("Data {:?}",data);
    //         let pixel = img.get_pixel_mut(x, y);

    //         *pixel = image::Rgb([*data,*data,*data]);
    //         x = x +1;
    //         if x == image_size {
    //             y = y +1;
    //             x = 0;
    //         }

    //         if y == image_size {
    //             img.save(format!("{}/out_{}.jpg",&cmd.output_path,seq)).unwrap();
    //             img = ImageBuffer::new(image_size,image_size);
    //             seq = seq +1;
    //             x = 0;
    //             y = 0;
    //         }
    //     });  
    //     output_handler.write_random(offset, &buf);
    //     offset = offset + 256;
    //     buf = vec![0;buf_size];
    // }

    // if x != image_size || y != image_size {
    //     loop {
    //         let pixel = img.get_pixel_mut(x, y);

    //         *pixel = image::Rgb([255,0,0]);
    //         x = x +1;
    //         if x == image_size {
    //             y = y +1;
    //             x = 0;
    //         }

    //         if y == image_size { 
    //             img.save(format!("{}/out_{}.jpg",&cmd.output_path,seq)).unwrap();
    //             break;
    //         }
    //     }
    // }


    let mut input_handler = InputHandler::new(&cmd.file_path);
    let mut output_handler = OutputHandler::new(&output_file);

    let mut image = image::open(&cmd.file_path).unwrap();
    let imgbuf = image.as_mut_rgb8().unwrap();

    buf = vec![0;buf_size];
    let mut buf_index = 0;
    offset = 0;
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        if x == 254 {
            output_handler.write_random(offset, &buf);
            buf_index = 0;
            buf = vec![0;buf_size];
            offset = offset + 256;
        }
        let data = pixel.0;
        if data[0] == data[1] && data[0] == data[2] {
            buf[buf_index] = data[0];
            buf_index = buf_index + 1;
        }
        // *pixel = image::Rgb([r, 0, b]);
    }
    
}
