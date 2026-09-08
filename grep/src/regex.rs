use regex::Regex;

#[derive(Debug, Clone)]
pub(crate) struct Regexes {
    pub normal: Regex,
    pub whole_word: Regex,
    pub ignore_case: Regex,
    pub whole_word_ignore_case: Regex,
}

pub(crate) fn create_regexes(key: &str) -> Regexes {
    let escaped = regex::escape(key);

    let normal = Regex::new(&escaped).unwrap();

    let whole_word = Regex::new(&format!(r"\b{}\b", escaped)).unwrap();

    let ignore_case = Regex::new(&format!(r"(?i){escaped}")).unwrap();

    let whole_word_ignore_case = Regex::new(&format!(r"(?i)\b{}\b", escaped)).unwrap();

    Regexes {
        normal,
        whole_word,
        ignore_case,
        whole_word_ignore_case,
    }
}
