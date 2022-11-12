use file::GoFile;
use import_matcher::ImportMatcher;
use sorter::ImportSorter;
use std::error::Error;

mod file;
mod import_matcher;
mod import_ranges;
mod sorter;

pub fn run(packages: Vec<String>, files: Vec<String>) -> Result<(), Box<dyn Error>> {
    let im = ImportMatcher::new()?;
    let is = ImportSorter::new(packages);

    for f in files {
        let mut gf = GoFile::read(is.clone(), &im, &f)?;
        println!("file {} lines count {}", f, gf.lines_count());
        gf.sort();
    }

    Ok(())
}
