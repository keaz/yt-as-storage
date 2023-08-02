use std::{
    fs::{self, File},
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
};

use log::debug;

pub struct InputHandler {
    file: File,
}

impl InputHandler {
    pub fn new(file_path: &String) -> Self {
        let path = Path::new(file_path);
        let file = File::open(path).unwrap();
        debug!("New file created {:?}", file);
        InputHandler { file }
    }

    pub fn read_random(&mut self, offset: u64, buf: &mut [u8]) -> usize {
        debug!("Reading offset {:?}",offset);
        self.file.seek(SeekFrom::Start(offset)).unwrap();
        let read_data = self.file.read(buf).unwrap();
        debug!("Data read {:?}",read_data);

        read_data   
    }
}

pub struct OutputHandler {
    file: File,
}

impl OutputHandler {
    pub fn new(file_path: &String) -> Self {
        let path = Path::new(file_path);
        if path.exists() {
            fs::remove_file(path).unwrap();
        }
        let file = File::create(path).unwrap();
        debug!("New file created {:?}", file);
        OutputHandler { file }
    }

    pub fn write_random(&mut self, offset: u64, buf: &[u8]) {
        self.file.seek(SeekFrom::Start(offset)).unwrap();
        self.file.write(buf).unwrap();
    }
}
