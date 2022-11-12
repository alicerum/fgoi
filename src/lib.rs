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
        let mut gf = match GoFile::read(is.clone(), &im, &f) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("could not process file '{}': {}", &f, e);
                continue;
            }
        };
        gf.sort();
        gf.write()?;
    }

    Ok(())
}
