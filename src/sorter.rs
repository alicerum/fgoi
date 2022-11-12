use super::import_ranges::Import;
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Clone, Debug, PartialOrd, Ord)]
pub enum ImportType {
    Core,
    ThirdParty,
    Custom(String),
}

#[derive(Clone)]
pub struct ImportSorter {
    buckets: HashMap<ImportType, Vec<Import>>,
}

impl ImportSorter {
    pub fn new(packages: Vec<String>) -> ImportSorter {
        let mut is = ImportSorter {
            buckets: HashMap::new(),
        };

        is.buckets.insert(ImportType::Core, Vec::new());
        is.buckets.insert(ImportType::ThirdParty, Vec::new());
        for p in packages {
            is.buckets.insert(ImportType::Custom(p), Vec::new());
        }

        is
    }

    pub fn insert(&mut self, i: Import) {
        let s = &i.url;
        if s.contains(".") && s.contains("/") {
            // try to insert custom import into the custom bucket
            for (k, v) in &mut self.buckets {
                if let ImportType::Custom(p) = k {
                    if s.starts_with(p) {
                        v.push(i);
                        return;
                    }
                }
            }

            // could not find custom bucket, let's insert into 3rd party one
            self.buckets
                .get_mut(&ImportType::ThirdParty)
                .unwrap()
                .push(i);
            return;
        }

        // otherwise, goes into the core import bucket
        self.buckets.get_mut(&ImportType::Core).unwrap().push(i);
    }

    pub fn get_buckets<'a>(&'a self) -> &'a HashMap<ImportType, Vec<Import>> {
        &self.buckets
    }

    pub fn sort(&mut self) {
        for (_, v) in &mut self.buckets {
            v.sort_by(|i1, i2| i1.url.cmp(&i2.url))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_custom() {
        let ps = vec!["github.com/ae".to_string(), "github.com/S1".to_string()];
        let mut is = ImportSorter::new(ps);

        is.insert(Import {
            name: None,
            url: "github.com/S1/hrechu".to_string(),
        });

        is.insert(Import {
            name: Some("hurpcoerc".to_string()),
            url: "github.com/S1/prchu".to_string(),
        });

        is.insert(Import {
            name: Some("alices".to_string()),
            url: "github.com/alice/very_project".to_string(),
        });

        assert_eq!(
            2,
            is.buckets
                .get(&ImportType::Custom("github.com/S1".to_string()))
                .unwrap()
                .len()
        );

        assert_eq!(1, is.buckets.get(&ImportType::ThirdParty).unwrap().len());
    }
}
