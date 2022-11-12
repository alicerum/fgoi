use crate::import_matcher::ImportMatcher;
use crate::import_ranges::Import;
use crate::sorter::ImportSorter;

use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct GoFile {
    lines: Vec<String>,
    is: ImportSorter,
}

impl GoFile {
    fn new(is: ImportSorter) -> GoFile {
        GoFile {
            lines: Vec::new(),
            is,
        }
    }

    fn add_line(&mut self, line: String) {
        self.lines.push(line);
    }

    pub fn read(
        is: ImportSorter,
        import_matcher: &ImportMatcher,
        path: &str,
    ) -> std::io::Result<GoFile> {
        let f = File::open(path)?;
        let r = BufReader::new(f);

        let mut is_inside_imports_block: bool = false;

        let mut gf = GoFile::new(is);
        for l in r.lines() {
            let line = l?;

            if is_inside_imports_block {
                if import_matcher.match_import_end(&line) {
                    is_inside_imports_block = false;
                    continue;
                }
                if let Some(i) = import_matcher.match_in_block(&line) {
                    gf.add_import(i);
                    continue;
                }
            }

            if let Some(i) = import_matcher.match_single(&line) {
                gf.add_import(i);
                continue;
            }

            if import_matcher.match_import_begin(&line) {
                is_inside_imports_block = true;
                continue;
            }

            gf.add_line(line);
        }

        Ok(gf)
    }

    pub fn sort(&mut self) {
        self.is.sort();
    }

    pub fn lines_count(&self) -> usize {
        self.lines.len()
    }

    fn add_import(&mut self, i: Import) {
        self.is.insert(i);
    }
}
