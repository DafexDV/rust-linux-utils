use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom},
    os::unix::fs::MetadataExt,
    path::Path,
};

/// Reads and gets the first 'n' bytes of a file
pub fn get_first_bytes(file_path: impl AsRef<Path>, n: usize) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buffer = vec![0u8; n];

    let bytes_read = file.read(&mut buffer)?;
    buffer.truncate(bytes_read);

    Ok(buffer)
}

/// Reads and gets the last 'n' bytes of a file
pub fn get_last_bytes(file_path: impl AsRef<Path>, n: usize) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;

    let file_size = file.metadata()?.size();
    let start = file_size.saturating_sub(n as u64);

    file.seek(SeekFrom::Start(start))?;

    let mut buffer = vec![0u8; n];

    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}
