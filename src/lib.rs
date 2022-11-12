use file::GoFile;
use import_matcher::ImportMatcher;
use sorter::ImportSorter;
use std::{
    error::Error,
    fs::{metadata, read_dir},
};

mod file;
mod import_matcher;
mod import_ranges;
mod sorter;

pub fn run(packages: Vec<String>, files: Vec<String>) -> Result<(), Box<dyn Error>> {
    let im = ImportMatcher::new()?;
    let is = ImportSorter::new(packages);

    for f in files {
        if let Err(e) = process_path(&f, &im, &is) {
            eprintln!("could not process file '{}': {}", &f, e);
        }
    }

    Ok(())
}

fn process_path(f: &str, im: &ImportMatcher, is: &ImportSorter) -> std::io::Result<()> {
    let m = metadata(f)?;
    if m.is_file() && f.ends_with(".go") {
        return process_file(f, im, is);
    } else if m.is_dir() {
        for path in read_dir(f)? {
            let path = path?.path();
            let s = path.to_str();
            if let Some(s) = s {
                if let Err(e) = process_path(s, im, is) {
                    eprintln!("could not process file '{}': {}", s, e);
                }
            }
        }
    }

    Ok(())
}

fn process_file(f: &str, im: &ImportMatcher, is: &ImportSorter) -> std::io::Result<()> {
    let mut gf = GoFile::read(is.clone(), im, f)?;
    gf.sort();
    gf.write()?;
    Ok(())
}
