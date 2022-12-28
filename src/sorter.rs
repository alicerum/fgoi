use super::import::Import;

#[derive(Clone)]
pub struct ImportSorter<'a> {
    core: Vec<Import>,
    third_party: Vec<Import>,
    custom: Vec<(&'a str, Vec<Import>)>,
}

impl<'a> ImportSorter<'a> {
    pub fn new(packages: Vec<&'a str>) -> ImportSorter<'a> {
        ImportSorter {
            core: vec![],
            third_party: vec![],
            custom: packages.into_iter().map(|p| (p, Vec::new())).collect(),
        }
    }

    /// Since we need to be able to pick up the best suitable
    /// bucket name for every import, this function is designed
    /// to do just that. It will pick the bucket name that starts
    /// with the desired name, but also the longest one of those
    /// suitable.
    /// it should have linear complexity, and since we do not expect
    /// many buckets to exist, should not make things complex at all.
    fn suitable_custom_bucket(&mut self, name: &str) -> Option<&mut Vec<Import>> {
        self.custom
            .iter_mut()
            .filter(|(bucket_name, _)| name.starts_with(*bucket_name))
            .max_by_key(|(bucket_name, _)| bucket_name.len())
            .map(|(_, bucket)| bucket)
    }

    fn custom_buckets_iter(&self) -> impl Iterator<Item = &Vec<Import>> {
        self.custom.iter().map(|(_, b)| b)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Vec<Import>> {
        [&self.core, &self.third_party]
            .into_iter()
            .chain(self.custom_buckets_iter())
    }

    pub fn insert(&mut self, i: impl Into<Import>) {
        let i = i.into();
        let s = &i.url;
        let bucket = if s.contains('.') && s.contains('/') {
            // try to insert custom import into the custom bucket
            if let Some(bucket) = self.suitable_custom_bucket(s) {
                bucket
            } else {
                // could not find custom bucket, let's insert into 3rd party one
                &mut self.third_party
            }
        } else {
            // otherwise, goes into the core import bucket
            &mut self.core
        };
        bucket.push(i);
    }

    pub fn sort(&mut self) {
        for v in [&mut self.core, &mut self.third_party]
            .into_iter()
            .chain(self.custom.iter_mut().map(|(_, b)| b))
        {
            v.sort_by(|i1, i2| i1.url.cmp(&i2.url));
        }
    }

    pub fn imports_count(&self) -> usize {
        self.core.len()
            + self.third_party.len()
            + self.custom_buckets_iter().map(Vec::len).sum::<usize>()
    }

    pub fn get_single_count(&self) -> Option<&Import> {
        self.iter().find(|v| v.len() == 1).map(|v| &v[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_custom() {
        let ps = vec!["github.com/ae", "github.com/S1"];
        let mut is = ImportSorter::new(ps);

        is.insert(("hurpcoerc", "github.com/S1/prchu"));
        is.insert((None, "github.com/S1/hrechu"));
        is.insert(("alices", "github.com/alice/very_project"));

        is.sort();

        let s1_imports = is
            .custom
            .iter()
            .find(|(n, _)| *n == "github.com/S1")
            .map(|(_, b)| b)
            .unwrap();
        assert_eq!(
            s1_imports,
            &[
                Import {
                    name: None,
                    url: "github.com/S1/hrechu".to_string()
                },
                Import {
                    name: Some("hurpcoerc".to_string()),
                    url: "github.com/S1/prchu".to_string()
                },
            ]
        );
        assert_eq!(is.imports_count(), 3);
        assert_eq!(is.third_party.len(), 1);
        assert_eq!(
            is.get_single_count(),
            Some(&Import {
                name: Some("alices".to_string()),
                url: "github.com/alice/very_project".to_string(),
            })
        );
    }
}
