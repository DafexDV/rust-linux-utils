use clap::Parser;

use crate::file_head::{FileHeads, Head, Mode, get_file_heads};

const DEFAULT_LINES: usize = 10;

#[derive(Parser, Debug)]
pub(crate) struct Cli {
    #[arg(
        short = 'c',
        help = "print the first NUM bytes of each file; with the leading '-', print all but the last NUM bytes of each file"
    )]
    pub bytes: Option<usize>,

    #[arg(
        short = 'n',
        help = "rint the first NUM lines instead of the first 10; with the leading '-', print all but the last NUM lines of each file"
    )]
    pub lines: Option<usize>,

    #[arg(short = 'q', help = "never print headers giving file names")]
    pub quiet: bool,

    #[arg(value_delimiter = ' ', num_args = 1..)]
    pub file_paths: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShowMode {
    Bytes,
    Lines,
}

impl From<Mode> for ShowMode {
    fn from(value: Mode) -> Self {
        match value {
            Mode::Bytes(_) => Self::Bytes,
            Mode::Lines(_) => Self::Lines,
        }
    }
}

#[derive(Debug)]
struct ShowOptions {
    quiet: bool,
    mode: ShowMode,
}

pub(crate) fn init_cli() {
    let cli = Cli::parse();

    let Cli {
        bytes,
        lines,
        quiet,
        file_paths,
    } = cli;

    let mode = bytes.map_or_else(
        || {
            let n: usize = lines.unwrap_or_else(|| DEFAULT_LINES);
            Mode::Lines(n)
        },
        |b| Mode::Bytes(b),
    );

    let file_heads = get_file_heads(&file_paths, mode);

    show_results(
        &file_heads,
        &ShowOptions {
            quiet,
            mode: mode.into(),
        },
    );
}

fn show_results(file_heads: &FileHeads, options: &ShowOptions) {
    if options.mode == ShowMode::Bytes {
        for file_head in file_heads.items() {
            match file_head.result() {
                Ok(Head::Bytes(bytes)) => {
                    if !options.quiet {
                        print!("====> {} <====\n", file_head.path().display());
                    }
                    print!("{}\n", String::from_utf8_lossy(bytes));
                }
                Ok(Head::Lines(_)) => unreachable!(),
                Err(error) => {
                    print!(
                        "rhead: cannot open '{}' for reading: {}\n",
                        file_head.path().display(),
                        error
                    );
                    break;
                }
            }
        }
    } else {
        for file_head in file_heads.items() {
            match file_head.result() {
                Ok(Head::Lines(lines)) => {
                    if !options.quiet {
                        print!("====> {} <====\n", file_head.path().display());
                    }
                    for line in lines {
                        println!("{}", line);
                    }
                }
                Ok(Head::Bytes(_)) => unreachable!(),
                Err(error) => {
                    print!(
                        "rhead: cannot open '{}' for reading: {}\n",
                        file_head.path().display(),
                        error
                    );
                    break;
                }
            }
        }
    }
}
