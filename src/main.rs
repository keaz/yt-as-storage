use std::env;

use clap::Parser;
use env_logger::Env;
use image::{Rgb, ImageBuffer, RgbImage, ImageFormat};
use log::{info, debug};
use yt_as_storage::{cmd::CmdArgs, io::{InputHandler, OutputHandler}};


fn main() {
    let cmd = CmdArgs::parse_from(env::args_os());
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    let mut input_handler = InputHandler::new(&cmd.file_path);
    let output_file = format!("{}/tibco",&cmd.output_path);
    let mut output_handler = OutputHandler::new(&output_file);
    let buf_size: usize = 255;
    let image_size: u32 = (buf_size).try_into().unwrap();
    let mut buf = vec![0;buf_size];
    let mut offset = 0;
    let mut pixcel_clunt = 0;

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
    //         println!("x::{:?} Data {:?}",x,data);
    //         pixcel_clunt = pixcel_clunt +1;
    //         let pixel = img.get_pixel_mut(x, y);

    //         *pixel = image::Rgb([*data,*data,*data]);
    //         x = x +1;
    //         if x == image_size {
    //             y = y +1;
    //             x = 0;
    //         }

    //         if y == image_size {
    //             img.save_with_format(format!("{}/out_{}.tiff",&cmd.output_path,seq), ImageFormat::Jpeg).unwrap();
    //             img = ImageBuffer::new(image_size,image_size);
    //             seq = seq +1;
    //             x = 0;
    //             y = 0;
    //         }
    //     });  
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
    //             // img.save(format!("{}/out_{}.jpg",&cmd.output_path,seq)).unwrap();
    //             img.save_with_format(format!("{}/out_{}.tiff",&cmd.output_path,seq), ImageFormat::Jpeg).unwrap();
    //             break;
    //         }
    //     }
    // }

    // debug!("Data in pixels {}",pixcel_clunt);

    let mut output_handler = OutputHandler::new(&output_file);
    
    let mut image = image::open(&cmd.file_path).unwrap();
    let imgbuf = image.as_mut_rgb8().unwrap();

    buf = vec![];
    
    offset = 0;
    let mut old_y = 0;
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // println!("X: {:?}",x);
        let buf_index:usize = x.try_into().unwrap();
        if y == old_y {
            println!("Data {:?}",buf);
            output_handler.write_random(offset, &buf);
            let ln = buf.len() as u64;
            offset = offset + ln;
            // buf_index = 0;
            buf = vec![];
            // 
            // old_y = y;
        }
        let data = pixel.0;
        // debug!("X::{},R {}, G {}, B {} ",x,data[0],data[1],data[2]);
        if data[0] == data[1] && data[0] == data[2] {
            pixcel_clunt = pixcel_clunt + 1;
            println!("X::{:?}, data[0] {:?}",x,data[0]);
            buf.push(data[0]);
            // buf.insert(buf_index, data[0]);
            // buf[buf_index] = data[0];
            // buf_index = buf_index + 1;
        }
        // *pixel = image::Rgb([r, 0, b]);
    }
    output_handler.write_random(offset, &buf);
    debug!("Data in pixels {}",pixcel_clunt);
    
}
