use std::{
    fs,
    path::{Path, PathBuf},
};

pub(crate) struct FileContent {
    path: PathBuf,
    content: Result<Vec<String>, String>,
}

pub(crate) struct ContentReport {
    items: Vec<FileContent>,
}

impl FileContent {
    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn content(&self) -> Result<&Vec<String>, &str> {
        match &self.content {
            Ok(lines) => Ok(lines),
            Err(error) => Err(error),
        }
    }
}

impl ContentReport {
    pub(crate) fn items(&self) -> &[FileContent] {
        &self.items
    }
}

pub(crate) fn read_file_contents(files: &Vec<String>) -> ContentReport {
    let mut items: Vec<FileContent> = Vec::new();

    for file in files {
        let path = Path::new(file);

        let content: Result<Vec<String>, String>;

        if path.exists() {
            match fs::read_to_string(path) {
                Ok(contents) => {
                    let mut lines: Vec<String> = Vec::new();

                    for l in contents.lines() {
                        lines.push(l.to_owned());
                    }

                    content = Ok(lines);
                }
                Err(error) => {
                    content = Err(error.to_string());
                }
            }
        } else {
            content = Err("No such file or directory".to_owned());
        }

        items.push(FileContent {
            path: path.into(),
            content,
        });
    }

    ContentReport { items }
}
