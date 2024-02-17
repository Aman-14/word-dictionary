use std::{
    fs::{File, OpenOptions},
    io,
    os::unix::prelude::FileExt,
};

use memmap2::{Mmap, MmapOptions};

use super::backend::StorageBackend;

const FILE_NAME: &str = "data/db.dat";

#[derive(Debug)]
pub struct FileBackend {
    file: File,
}

impl StorageBackend for FileBackend {
    fn new() -> Self {
        return Self {
            file: File::open(FILE_NAME).unwrap(),
        };
    }
    fn read_at(&self, buf: &mut [u8], offset: u64) -> Result<usize, std::io::Error> {
        return self.file.read_at(buf, offset);
    }

    fn write_at(&self, buf: &[u8], offset: u64) -> Result<usize, std::io::Error> {
        return self.file.write_at(buf, offset);
    }
}

// pub struct MMapBackend {
//     buf: Mmap,
//     file: File,
// }
//
// impl StorageBackend for MMapBackend {
//     fn new() -> Self {
//         let file = OpenOptions::new()
//             .write(true)
//             .read(true)
//             .open(FILE_NAME)
//             .unwrap();
//
//         let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
//         return Self { buf: mmap, file };
//     }
//
//     fn read_at(&self, buf: &mut [u8], offset: u64) -> Result<usize, std::io::Error> {
//         let offset = offset as usize;
//         let data = self.buf.get(offset..(buf.len() + offset));
//         if let Some(data) = data {
//             buf.copy_from_slice(data);
//             return Ok(buf.len());
//         }
//         return Err(io::Error::new(io::ErrorKind::NotFound, "Data is empty"));
//     }
//
//     fn write_at(&self, buf: &[u8], offset: u64) -> Result<usize, std::io::Error> {
//         return self.file.write_at(buf, offset);
//     }
// }
