use std::{
    fs, io,
    path::{Path, PathBuf},
};

use regex::Regex;
use walkdir::WalkDir;

use crate::regex::create_regexes;

pub(crate) struct PatternSearchOptions {
    pub ignore_case: bool,
    pub invert_match: bool,
    pub whole_word: bool,
}

pub(crate) struct SearchMatch {
    pub line_number: usize,
    pub line_content: String,

    pub start: usize,
    pub end: usize,
}

pub(crate) struct PatternSearchResult {
    pub file_path: PathBuf,
    pub matches: Vec<SearchMatch>,
}

impl PatternSearchResult {
    fn is_empty(&self) -> bool {
        self.matches.is_empty()
    }
}

pub(crate) fn search_patterns(
    path: &Path,
    pattern_str: &str,
    options: PatternSearchOptions,
) -> Result<Vec<PatternSearchResult>, io::Error> {
    let regex = get_regex_by_options(pattern_str, &options);

    let mut results: Vec<PatternSearchResult> = Vec::new();

    if path.is_dir() {
        for entry in WalkDir::new(path) {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let result = search_in_file(path, &regex, &options)?;
                if !result.is_empty() {
                    results.push(result);
                }
            }
        }
    } else {
        let result = search_in_file(path, &regex, &options)?;
        if !result.is_empty() {
            results.push(result);
        }
    }

    Ok(results)
}

fn search_in_file(
    path: &Path,
    regex: &Regex,
    options: &PatternSearchOptions,
) -> Result<PatternSearchResult, io::Error> {
    let mut matches: Vec<SearchMatch> = Vec::new();

    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => {
            return Ok(PatternSearchResult {
                file_path: path.into(),
                matches,
            });
        }
    };

    for (n, line) in content.lines().enumerate() {
        let regex_matches: Vec<_> = regex.find_iter(line).collect();

        if options.invert_match {
            if regex_matches.is_empty() {
                matches.push(SearchMatch {
                    line_number: n + 1,
                    line_content: line.to_string(),
                    start: 0,
                    end: line.len(),
                });
            }
            continue;
        }

        for rm in regex_matches {
            matches.push(SearchMatch {
                line_number: n + 1,
                line_content: line.to_string(),
                start: rm.start(),
                end: rm.end(),
            });
        }
    }

    Ok(PatternSearchResult {
        file_path: path.into(),
        matches,
    })
}

fn get_regex_by_options(pattern_str: &str, options: &PatternSearchOptions) -> Regex {
    let regexes = create_regexes(pattern_str);

    return match (options.whole_word, options.ignore_case) {
        (false, false) => regexes.normal,
        (false, true) => regexes.ignore_case,
        (true, false) => regexes.whole_word,
        (true, true) => regexes.whole_word_ignore_case,
    };
}
