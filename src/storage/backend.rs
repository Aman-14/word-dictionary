use std::io;

pub trait StorageBackend {
    // async fn read_exact(&mut self, but: &mut [u8]) -> Result<(), io::Error>;
    fn new() -> Self;
    fn read_at(&self, buf: &mut [u8], offset: u64) -> Result<usize, io::Error>;
    fn write_at(&self, buf: &[u8], offset: u64) -> Result<usize, io::Error>;
}
