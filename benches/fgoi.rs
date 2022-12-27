use criterion::{criterion_group, criterion_main, Criterion};
use fgoi::{GoFile, Import, ImportMatcher, ImportSorter};
use std::fs;
use tempfile::tempdir;

const SAMPLE_GO_FILE: &str = "benches/sample.go";

fn go_file(c: &mut Criterion) {
    let ps = vec!["github.com/ae", "github.com/S1"];
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
    is.sort();
    let im = ImportMatcher::new().unwrap();
    c.bench_function("read", |b| {
        b.iter(|| GoFile::read(is.clone(), &im, SAMPLE_GO_FILE).unwrap())
    });

    let dir = tempdir().unwrap();
    let file_path = dir.path().join("output.go");
    fs::copy(SAMPLE_GO_FILE, file_path.clone()).unwrap();
    let gf = GoFile::read(is, &im, file_path.to_str().unwrap()).unwrap();
    c.bench_function("write", |b| b.iter(|| gf.write().unwrap()));
}

criterion_group!(benches, go_file);
criterion_main!(benches);
