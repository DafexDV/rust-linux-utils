use std::path::Path;

use clap::Parser;
use owo_colors::OwoColorize;

use crate::search::{PatternSearchOptions, PatternSearchResult, search_patterns};

/// Main struct for clap to put the cli argument values
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    // Searches for the pattern regardless of uppercase or lowercase.
    #[arg(short = 'i', help = "Search ignoring uppercase and lowercase")]
    pub ignore_case: bool,

    // Displays only the lines that do not contain the pattern.
    #[arg(short = 'v', help = "Show lines that do not match")]
    pub invert_match: bool,

    // Instead of showing the lines, it just tells you how many matches were found.
    #[arg(
        short = 'c',
        help = "Instead of showing the lines, it just tells you how many matches were found"
    )]
    pub count: bool,

    // Only matches the exact word, not if it is part of a larger
    #[arg(
        short = 'w',
        help = "Only matches the exact word, not if it is part of a larger"
    )]
    pub whole_word: bool,

    #[arg(help = "The file in which the pattern will be searched")]
    pub file: String,

    #[arg(help = "Pattern to search")]
    pub pattern: String,
}

struct ShowOptions {
    count: bool,
}

pub(crate) fn init_cli() {
    let cli = Cli::parse();

    let file = &cli.file;
    let pattern = &cli.pattern;

    let path = Path::new(file);

    let search_options = PatternSearchOptions {
        ignore_case: cli.ignore_case,
        invert_match: cli.invert_match,
        whole_word: cli.whole_word,
    };

    let show_options = ShowOptions { count: cli.count };

    match search_patterns(path, pattern, search_options) {
        Ok(results) => {
            show_results(&results, show_options);
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}

fn show_results(results: &Vec<PatternSearchResult>, options: ShowOptions) {
    if options.count {
        let count = results.iter().map(|i| i.matches.len()).len();
        println!("{}", count);
        return;
    }

    for result in results {
        let file_path = &result.file_path;
        println!("{:?}", file_path.cyan());

        for m in &result.matches {
            let before = &m.line_content[..m.start];
            let matched = &m.line_content[m.start..m.end];
            let after = &m.line_content[m.end..];

            print!("\t");
            print!("{}", format!("{}.", m.line_number).yellow());
            print!("\t");
            print!("{}", before.bright_black());
            print!("{}", matched.green());
            print!("{}", after.bright_black());
            print!("\n");
        }
    }
}
