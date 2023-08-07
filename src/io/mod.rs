use image::{
    imageops::{resize, Nearest},
    Rgb, RgbImage,
};
use log::debug;
use std::{
    fs::{self, File, ReadDir},
    io::{Error, Read, Seek, SeekFrom},
    path::Path,
    str,
    sync::{Arc, Mutex},
};

pub struct InputHandler {
    file: Arc<Mutex<File>>,
}

impl Clone for InputHandler {
    fn clone(&self) -> Self {
        Self {
            file: self.file.clone(),
        }
    }
}

impl InputHandler {
    pub fn new(file_path: &String) -> Self {
        let path = Path::new(file_path);
        let file = File::open(path).unwrap();
        debug!("New file opened {:?}", file);
        InputHandler {
            file: Arc::new(Mutex::new(file)),
        }
    }

    pub fn get_file_size(&mut self) -> u64 {
        self.file.lock().unwrap().metadata().unwrap().len()
        // self.file.metadata().unwrap().len()
    }

    pub fn read_input_data(&mut self, offset: u64, buf: &mut [u8]) -> usize {
        // debug!("Reading offset {:?}",offset);
        let file = self.file.clone();
        let mut file = file.lock().unwrap();
        (*file).seek(SeekFrom::Start(offset)).unwrap();
        let read_data = file.read(buf).unwrap();
        // debug!("Data read {:?}",read_data);

        read_data
    }
}

pub struct OutputHandler {}

impl OutputHandler {
    pub fn encode_frames(buf: &[u8], img_index: &str) {
        let mut img = RgbImage::new(256, 144);

        let mut all_bits = "".to_string();
        buf.iter().for_each(|&bit| {
            let each_bit = format!("{:08b}", bit);
            all_bits.push_str(&each_bit);
        });

        'outer: for y in 0..144 {
            for x in 0..256 {
                if 0 < all_bits.len() {
                    let current_bit = if all_bits.remove(0) == '1' { 1 } else { 0 };
                    img.put_pixel(
                        x,
                        y,
                        Rgb([255 * current_bit, 255 * current_bit, 255 * current_bit]),
                    );
                } else {
                    img.put_pixel(x, y, Rgb([255, 0, 0]));
                    break 'outer;
                }
            }
        }

        let img_scaled = resize(&img, 1280, 720, Nearest);

        let img_name = &format!("vid2fps/output{}.png", img_index);

        // write it out to a file
        img_scaled.save(img_name).unwrap();
    }

    pub fn decode_frames(img_index: &str) -> String {
        let mut img_name: String = "vid2fps/extracted".to_owned();
        img_name.push_str(img_index);
        img_name.push_str(".png");

        let img = image::open(img_name).unwrap();

        let img_scaled = resize(&img, 256, 144, Nearest);

        let mut all_bits = "".to_string();

        let mut pushed_bits = 0;

        'outer: for y in 0..144 {
            for x in 0..256 {
                let mut s = img_scaled.get_pixel(x, y).to_owned();

                for i in 0..3 {
                    if s[i] > 128 {
                        s[i] = 255;
                    } else {
                        s[i] = 0;
                    }
                }

                if s[0] == s[1] && s[1] == s[2] && s[2] == 0 {
                    all_bits.push_str("0");
                    pushed_bits += 1;
                } else if s[0] == 255 && s[1] == s[2] && s[2] == 0 {
                    break 'outer;
                } else {
                    all_bits.push_str("1");
                    pushed_bits += 1;
                }

                if pushed_bits == 8 {
                    all_bits.push_str(" ");
                    pushed_bits = 0;
                }
            }
        }

        bin_str_to_word(all_bits.as_str().trim_end())
    }
}

fn bin_str_to_word(bin_str: &str) -> String {
    bin_str
        .split(" ")
        .map(|n| u32::from_str_radix(n, 2).unwrap())
        .map(|c| char::from_u32(c).unwrap())
        .collect()
}

pub fn clear_vid2fps() {
    let vid2fps_dir = fs::read_dir("vid2fps");
    delete_dir_contents(vid2fps_dir);
}

pub fn clear_vidout() {
    let vidout_dir = fs::read_dir("vidout");
    delete_dir_contents(vidout_dir);
}

pub fn create_dirs(dirs: Vec<&str>) {
    for dir in dirs {
        fs::create_dir_all(dir).expect(format!("Failed to create {} dir", dir).as_str());
    }
}

pub fn get_first_in_dir(read_dir_res: Result<ReadDir, Error>) -> String {
    let mut out: String = "".to_string();
    if let Ok(dir) = read_dir_res {
        for entry in dir {
            out = entry.unwrap().path().display().to_string();
            break;
        }
    };

    if out.len() == 0 {
        panic!("Nothing in the vidin directory.");
    }

    out
}

fn delete_dir_contents(read_dir_res: Result<ReadDir, Error>) {
    if let Ok(dir) = read_dir_res {
        for entry in dir {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_dir() {
                    fs::remove_dir_all(path).expect("Failed to remove a dir");
                } else {
                    fs::remove_file(path).expect("Failed to remove a file");
                }
            };
        }
    };
}

pub struct Data {
    pub index: u64,
    pub buf: Vec<u8>,
}

impl Data {
    pub fn new(index: u64, buf: Vec<u8>) -> Self {
        Data { index, buf }
    }
}

impl Clone for Data {
    fn clone(&self) -> Self {
        Self {
            index: self.index.clone(),
            buf: self.buf.clone(),
        }
    }
}
