use std::{
    fs,
    path::{Path, PathBuf},
};

pub(crate) struct Statistics {
    pub line_count: usize,
    pub word_count: usize,
    pub byte_count: usize,
}

pub(crate) struct FileStatistics {
    path: PathBuf,
    result: Result<Statistics, String>,
}

pub(crate) struct StatisticsReport {
    items: Vec<FileStatistics>,
}

impl FileStatistics {
    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn result(&self) -> Result<&Statistics, &str> {
        match &self.result {
            Ok(statistics) => Ok(&statistics),
            Err(error) => Err(&error),
        }
    }
}

impl StatisticsReport {
    pub(crate) fn items(&self) -> &[FileStatistics] {
        &self.items
    }

    pub(crate) fn get_total_statistics(&self) -> Statistics {
        let mut line_count: usize = 0;
        let mut word_count: usize = 0;
        let mut byte_count: usize = 0;

        for item in self.items.iter().filter_map(|item| match &item.result {
            Ok(statistics) => Some(statistics),
            Err(_) => None,
        }) {
            line_count += item.line_count;
            word_count += item.word_count;
            byte_count += item.byte_count;
        }

        Statistics {
            line_count,
            word_count,
            byte_count,
        }
    }

    pub(crate) fn get_column_width(&self) -> (usize, usize, usize) {
        self.items
            .iter()
            .filter_map(|item| item.result.as_ref().ok())
            .fold((1, 1, 1), |(line_width, word_width, byte_width), item| {
                (
                    line_width.max(item.line_count.to_string().len()),
                    word_width.max(item.word_count.to_string().len()),
                    byte_width.max(item.byte_count.to_string().len()),
                )
            })
    }
}

pub(crate) fn get_statistics_report(files: &Vec<String>) -> StatisticsReport {
    let mut items: Vec<FileStatistics> = Vec::new();

    for file in files {
        let path = Path::new(file);

        let result: Result<Statistics, String>;

        if path.exists() {
            match fs::read_to_string(path) {
                Ok(contents) => {
                    let line_count = contents.lines().count();
                    let word_count = contents.split_whitespace().count();
                    let byte_count = contents.bytes().count();

                    result = Ok(Statistics {
                        line_count,
                        word_count,
                        byte_count,
                    })
                }
                Err(error) => result = Err(error.to_string()),
            }
        } else {
            result = Err("No such file or directory".to_owned())
        }

        items.push(FileStatistics {
            path: path.into(),
            result,
        });
    }

    StatisticsReport { items }
}
