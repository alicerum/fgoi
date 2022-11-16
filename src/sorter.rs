use super::import::Import;
use std::{collections::HashMap, rc::Rc};

#[derive(Eq, Hash, PartialEq, Clone, Debug, PartialOrd, Ord)]
pub enum ImportType {
    Core,
    ThirdParty,
    Custom(Rc<String>),
}

#[derive(Clone)]
pub struct ImportSorter {
    original: Vec<Rc<String>>,
    buckets: HashMap<ImportType, Vec<Import>>,
}

enum SorterOrder {
    Core,
    ThirdParty,
    Custom,
}

/// Iterator object over the import buckets in the sorter.
/// It releases import buckets one by one in the original order
/// specified by the user via command line arguments.
/// First it receives Core, then ThirdParty buckets, and only
/// then all the custom buckets specified by the user order.
pub struct ImportSorterIter<'a> {
    current_key: SorterOrder,
    current_custom: usize,
    sorter: &'a ImportSorter,
}

impl<'a> Iterator for ImportSorterIter<'a> {
    type Item = &'a Vec<Import>;
    fn next(&mut self) -> Option<Self::Item> {
        // Lets go through all the possible bucket types
        let (new_key, bucket) = match self.current_key {
            // for core and third party buckets just return those, and only then
            // deal with the order of the custom ones
            SorterOrder::Core => (SorterOrder::ThirdParty, Some(ImportType::Core)),
            SorterOrder::ThirdParty => (SorterOrder::Custom, Some(ImportType::ThirdParty)),
            SorterOrder::Custom => {
                // if theres no custom buckets, or if custom bucket is gone through,
                // then just return nothing.
                // otherwise, get the correct package name and create bucket key from it
                if self.current_custom == self.sorter.original.len() {
                    (SorterOrder::Custom, None)
                } else {
                    let package = self.sorter.original[self.current_custom].clone();
                    self.current_custom += 1;
                    (SorterOrder::Custom, Some(ImportType::Custom(package)))
                }
            }
        };

        // after correct bucket key has been obtained, return this bucket.
        // in case no bucket key was found, return nothing. it is efficiently
        // the end of the iterator
        self.current_key = new_key;
        bucket.map(|k| &self.sorter.buckets[&k])
    }
}

impl ImportSorter {
    pub fn new(packages: Vec<Rc<String>>) -> ImportSorter {
        let mut is = ImportSorter {
            original: packages,
            buckets: HashMap::new(),
        };

        is.buckets.insert(ImportType::Core, Vec::new());
        is.buckets.insert(ImportType::ThirdParty, Vec::new());
        for p in &is.original {
            is.buckets.insert(ImportType::Custom(p.clone()), Vec::new());
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
    fn suitable_custom_bucket_name(&self, name: &str) -> Option<Rc<String>> {
        let mut suitable_names: Vec<Rc<String>> = Vec::new();
        for k in self.buckets.keys() {
            if let ImportType::Custom(bucket_name) = k {
                if name.starts_with(bucket_name.as_ref()) {
                    suitable_names.push(bucket_name.clone());
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
        Some(suitable_names[index].clone())
    }

    pub fn iter(&self) -> ImportSorterIter {
        ImportSorterIter {
            current_key: SorterOrder::Core,
            current_custom: 0,
            sorter: self,
        }
    }

    pub fn insert(&mut self, i: Import) {
        let s = &i.url;
        if s.contains(".") && s.contains("/") {
            // try to insert custom import into the custom bucket
            if let Some(bn) = self.suitable_custom_bucket_name(&s) {
                if let Some(bucket) = self.buckets.get_mut(&ImportType::Custom(bn.clone())) {
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
        let ps = vec![
            Rc::new("github.com/ae".to_string()),
            Rc::new("github.com/S1".to_string()),
        ];
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
                .get(&ImportType::Custom(Rc::new("github.com/S1".to_string())))
                .unwrap()
                .len()
        );

        assert_eq!(1, is.buckets.get(&ImportType::ThirdParty).unwrap().len());
    }
}
