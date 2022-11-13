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

    /// Since we need to be able to pick up the best suitable
    /// bucket name for every import, this function is designed
    /// to do just that. It will pick the bucket name that starts
    /// with the desired name, but also the longest one of those
    /// suitable.
    /// it should have linear complexity, and since we do not expect
    /// many buckets to exist, should not make things complex at all.
    fn suitable_custom_bucket_name(&self, name: &str) -> Option<&str> {
        let mut suitable_names: Vec<&str> = Vec::new();
        for k in self.buckets.keys() {
            if let ImportType::Custom(bucket_name) = k {
                if name.starts_with(bucket_name) {
                    suitable_names.push(&bucket_name);
                }
            }
        }
        if suitable_names.len() == 0 {
            return None;
        }
        let mut len = 0;
        let mut index = 0;
        for (i, n) in suitable_names.iter().enumerate() {
            if n.len() > len {
                len = n.len();
                index = i;
            }
        }
        Some(suitable_names[index])
    }

    pub fn insert(&mut self, i: Import) {
        let s = &i.url;
        if s.contains(".") && s.contains("/") {
            // try to insert custom import into the custom bucket
            if let Some(bn) = self.suitable_custom_bucket_name(&s) {
                if let Some(bucket) = self.buckets.get_mut(&ImportType::Custom(bn.to_string())) {
                    bucket.push(i);
                    return;
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

    pub fn imports_count(&self) -> usize {
        let mut count: usize = 0;
        for (_, v) in &self.buckets {
            count += v.len();
        }

        count
    }

    pub fn get_single_count(&self) -> Option<&Import> {
        for (_, v) in &self.buckets {
            if v.len() == 1 {
                return v.get(0);
            }
        }
        None
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
