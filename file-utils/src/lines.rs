use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

/// Reads and gets the 'n' lines of a file
pub fn get_first_lines(file_path: impl AsRef<Path>, n: usize) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    reader.lines().take(n).collect()
}

/// Reads a file and gets its 'n' last lines
pub fn get_last_lines(file_path: impl AsRef<Path>, n: usize) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut lines = VecDeque::with_capacity(n);

    for line in reader.lines() {
        let line = line?;

        if lines.len() == n {
            lines.pop_front();
        }

        lines.push_back(line);
    }

    Ok(lines.into_iter().collect())
}
