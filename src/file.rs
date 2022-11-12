use crate::import_matcher::ImportMatcher;
use crate::import_ranges::Import;
use crate::sorter::{ImportSorter, ImportType};
use itertools::Itertools;

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};

pub struct GoFile {
    path: String,
    lines: Vec<String>,
    is: ImportSorter,
}

impl GoFile {
    fn new(is: ImportSorter, path: String) -> GoFile {
        GoFile {
            path,
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
        let f = File::open(&path)?;
        let r = BufReader::new(f);

        let mut is_inside_imports_block: bool = false;

        let mut gf = GoFile::new(is, path.to_string());
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
                } else if line.trim().len() > 0 {
                    // line is not empty, but also it is not
                    // an import line. something is clearly wrong
                    // here, better ignore file
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "File is not correct",
                    ));
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

    fn add_import(&mut self, i: Import) {
        self.is.insert(i);
    }

    pub fn write(&self) -> std::io::Result<()> {
        let f = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.path)?;
        let mut lw = BufWriter::new(&f);
        let mut counter: usize = 0;
        let mut after_import = false;
        let mut new_size: usize = 0;

        for l in &self.lines {
            counter += 1;

            if self.is.do_imports_exist() && counter == 3 {
                lw.write("import (\n".as_bytes())?;
                let mut put_blank = false;

                for k in self.is.get_buckets().keys().sorted() {
                    let v = self.is.get_buckets().get(k).unwrap();
                    // if we need to put blank and next block is longer than 0
                    if put_blank && v.len() > 0 {
                        new_size += lw.write("\n".as_bytes())?;
                    }
                    new_size += write_bucket(&mut lw, v)?;
                    if v.len() > 0 {
                        // if we have written something into imports
                        // block, then for the next block put line
                        put_blank = true;
                    }
                }

                new_size += lw.write(")\n\n".as_bytes())?;
                after_import = true;
                continue;
            }

            // this is going to only be triggered after the import
            // block is filled
            // only one empty line should appear after the import
            if after_import {
                if l.trim().len() == 0 {
                    continue;
                } else {
                    after_import = false;
                }
            }
            new_size += lw.write(format!("{}\n", l).as_bytes())?;
        }

        // set new length to the file (it might be less than it used to
        // be before the rewriting
        f.set_len(new_size as u64)?;

        Ok(())
    }
}

fn write_bucket(bf: &mut BufWriter<&File>, imports: &Vec<Import>) -> std::io::Result<usize> {
    let mut written: usize = 0;
    for i in imports {
        if let Some(n) = &i.name {
            written += bf.write(format!("\t{} \"{}\"\n", n, i.url).as_bytes())?;
        } else {
            written += bf.write(format!("\t\"{}\"\n", i.url).as_bytes())?;
        }
    }

    Ok(written)
}
