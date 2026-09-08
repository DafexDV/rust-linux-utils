use clap::Parser;

use crate::file_statistics::{Statistics, StatisticsReport, get_statistics_report};

#[derive(Parser, Debug)]
pub(crate) struct Cli {
    #[arg(value_delimiter = ' ', num_args=1..)]
    pub files: Vec<String>,
}

fn show_report(report: &StatisticsReport) {
    let (line_width, word_width, byte_width) = report.get_column_width();

    for item in report.items() {
        match item.result() {
            Ok(Statistics {
                line_count,
                word_count,
                byte_count,
            }) => {
                println!(
                    "{:>line_width$} {:>word_width$} {:>byte_width$} {}",
                    line_count,
                    word_count,
                    byte_count,
                    item.path().display()
                );
            }
            Err(error) => println!("rwc: {}: {}", item.path().display(), error),
        }
    }

    if report.items().len() > 1 {
        let Statistics {
            line_count,
            word_count,
            byte_count,
        } = report.get_total_statistics();

        println!(
            "{:>line_width$} {:>word_width$} {:>byte_width$} {}",
            line_count, word_count, byte_count, "total"
        );
    }
}

pub(crate) fn init_cli() {
    let cli = Cli::parse();

    let Cli { files } = &cli;

    let statistics_report = get_statistics_report(files);

    show_report(&statistics_report);
}
