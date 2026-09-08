use clap::Parser;

use crate::file_content::{ContentReport, read_file_contents};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[arg(short = 'n', help = "number all output lines")]
    pub number: bool,

    #[arg(short = 'E', help = "display $ or ^M$ at the end of each line")]
    pub show_ends: bool,

    #[arg(value_delimiter = ' ', num_args = 1..)]
    pub files: Vec<String>,
}

struct ShowOptions {
    show_number: bool,
    show_ends: bool,
}

pub fn init_cli() {
    let cli = Cli::parse();

    let Cli {
        number,
        show_ends,
        files,
    } = cli;

    let content_report = read_file_contents(&files);

    show_content_report(
        &content_report,
        ShowOptions {
            show_number: number,
            show_ends,
        },
    );
}

fn show_content_report(content_report: &ContentReport, options: ShowOptions) {
    // global line number because the original grep
    // doesn't reset the line count on multiple file
    // read
    let mut line_number = 0;
    for file_content in content_report.items() {
        match file_content.content() {
            Ok(lines) => {
                for line in lines.iter() {
                    let mut modified_line: String = line.clone();
                    if options.show_ends {
                        modified_line.push_str("$");
                    }

                    if options.show_number {
                        line_number += 1;
                        print!("\t{}.\t", line_number);
                    }
                    print!("{}", modified_line);
                    print!("\n");
                }
            }
            Err(error) => {
                println!("rcat: {}: {}", file_content.path().display(), error)
            }
        }
    }
}
