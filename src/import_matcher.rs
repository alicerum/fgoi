use super::import_ranges::Import;
use regex::{Error, Regex};

pub struct ImportMatcher {
    single_import: Regex,
    block_import: Regex,
    import_begin: Regex,
    import_end: Regex,
}

impl ImportMatcher {
    pub fn new() -> Result<ImportMatcher, Error> {
        Ok(ImportMatcher {
            single_import: Regex::new("^\\s*import\\s+([a-zA-Z-_0-9]*)?\\s*\"(.+)\"\\s*$")?,
            block_import: Regex::new("^\\s*([a-zA-Z-_0-9]*)?\\s*\"(.+)\"\\s*$")?,
            import_begin: Regex::new("^\\s*import\\s+\\(\\s*$")?,
            import_end: Regex::new("^\\s*\\)\\s*$")?,
        })
    }

    pub fn match_single(&self, s: &str) -> Option<Import> {
        for cap in self.single_import.captures_iter(s) {
            let import = Import {
                name: string_from_match(&cap[1]),
                url: String::from(&cap[2]),
            };
            return Option::Some(import);
        }
        Option::None
    }

    pub fn match_in_block(&self, s: &str) -> Option<Import> {
        for cap in self.block_import.captures_iter(s) {
            let import = Import {
                name: string_from_match(&cap[1]),
                url: String::from(&cap[2]),
            };
            return Option::Some(import);
        }
        Option::None
    }

    pub fn match_import_begin(&self, s: &str) -> bool {
        self.import_begin.is_match(s)
    }

    pub fn match_import_end(&self, s: &str) -> bool {
        self.import_end.is_match(s)
    }
}

fn string_from_match(s: &str) -> Option<String> {
    if s.len() == 0 {
        return Option::None;
    }
    Option::Some(String::from(s))
}
