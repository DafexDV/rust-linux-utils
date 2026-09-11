use std::path::{Path, PathBuf};

use file_utils::{bytes::get_first_bytes, lines::get_first_lines};

#[derive(Debug)]
pub(crate) enum Head {
    Lines(Vec<String>),
    Bytes(Vec<u8>),
}

#[derive(Debug)]
pub(crate) struct FileHead {
    path: PathBuf,
    result: Result<Head, String>,
}

#[derive(Debug)]
pub(crate) struct FileHeads {
    items: Vec<FileHead>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Mode {
    Lines(usize),
    Bytes(usize),
}

impl FileHead {
    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn result(&self) -> Result<&Head, &String> {
        self.result.as_ref()
    }
}

impl FileHeads {
    pub(crate) fn items(&self) -> &[FileHead] {
        &self.items
    }
}

pub(crate) fn get_file_heads(file_paths: &Vec<impl AsRef<Path>>, mode: Mode) -> FileHeads {
    let mut items: Vec<FileHead> = Vec::new();

    for file_path in file_paths {
        let result = match mode {
            Mode::Bytes(n) => match get_first_bytes(file_path, n) {
                Ok(bytes) => Ok(Head::Bytes(bytes)),
                Err(error) => Err(error.to_string()),
            },
            Mode::Lines(n) => match get_first_lines(file_path, n) {
                Ok(lines) => Ok(Head::Lines(lines)),
                Err(error) => Err(error.to_string()),
            },
        };

        items.push(FileHead {
            path: file_path.as_ref().into(),
            result,
        });
    }

    FileHeads { items }
}
