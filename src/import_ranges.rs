struct ImportRange {
    start: usize,
    end: usize,
}

#[derive(Clone)]
pub struct Import {
    pub name: Option<String>,
    pub url: String,
}

pub struct ImportRanges {
    ranges: Vec<ImportRange>,
}

impl ImportRanges {
    pub fn new() -> ImportRanges {
        ImportRanges { ranges: Vec::new() }
    }

    pub fn add_range(&mut self, start: usize, end: usize) {
        self.ranges.push(ImportRange { start, end });
    }

    pub fn is_in_range(&self, line: usize) -> bool {
        for r in &self.ranges {
            if line >= r.start && line <= r.end {
                return true;
            }
        }

        false
    }
}
