use std::{
    fs::{self, File},
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
};

use log::debug;

pub struct FileHandler {
    file: File,
}

impl FileHandler {

    pub fn new(file_path: &String) -> Self{
        let path = Path::new(file_path);
        // if path.exists() {
        //     fs::remove_file(path).unwrap();
        // }
        // let file = File::create(path).unwrap();
        let file = File::open(path).unwrap();
        debug!("New file created {:?}", file);
        FileHandler{
            file
        }
    }


    pub fn write_random(&mut self, offset: u64, buf: &[u8]) {
        self.file.seek(SeekFrom::Start(offset)).unwrap();
        self.file.write(buf).unwrap();
    }

    pub  fn read_random(&mut self, offset: u64, buf: &mut [u8]) -> bool {
        self.file.seek(SeekFrom::Start(offset)).unwrap();
        let read_data = self.file.read(buf).unwrap();

        read_data == 0
    }

}
