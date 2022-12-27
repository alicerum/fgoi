use crate::import::Import;
use crate::matcher::ImportMatcher;
use crate::sorter::ImportSorter;

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};

pub struct GoFile<'a> {
    path: String,
    lines: Vec<String>,
    is: ImportSorter<'a>,
}

impl<'a> GoFile<'a> {
    fn new(is: ImportSorter<'a>, path: String) -> GoFile<'a> {
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
        is: ImportSorter<'a>,
        import_matcher: &ImportMatcher,
        path: &str,
    ) -> std::io::Result<GoFile<'a>> {
        let f = File::open(path)?;
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
                } else if !line.trim().is_empty() {
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
        let new_size = self.write_to(&mut lw)?;
        // set new length to the file (it might be less than it used to
        // be before the rewriting
        f.set_len(new_size as u64)?;
        Ok(())
    }

    pub fn write_to<W: Write>(&self, mut writer: W) -> std::io::Result<usize> {
        let mut counter: usize = 0;
        let mut after_import = false;
        let mut new_size: usize = 0;
        let mut import_line: usize = 3;

        for l in &self.lines {
            counter += 1;

            if !after_import && l.starts_with("package ") {
                import_line = counter + 2;
            }

            // if only one import exists, then get this single import
            // and print it into the file as is
            if self.is.imports_count() == 1 && counter == import_line {
                if let Some(i) = self.is.get_single_count() {
                    new_size += writer.write(format!("import {}\n\n", i).as_bytes())?;
                }
                after_import = true;
            } else if self.is.imports_count() > 0 && counter == import_line {
                // else, if multiple imports exist, then we need to be smarter about them
                // and print them in a very specific way
                writer.write_all("import (\n".as_bytes())?;
                let mut put_blank = false;

                // writing all buckets here now
                for v in self.is.iter() {
                    // if we need to put blank and next block is longer than 0
                    if put_blank && !v.is_empty() {
                        new_size += writer.write(b"\n")?;
                    }
                    new_size += write_bucket(&mut writer, v)?;
                    if !v.is_empty() {
                        // if we have written something into imports
                        // block, then for the next block put line
                        put_blank = true;
                    }
                }

                new_size += writer.write(")\n\n".as_bytes())?;
                after_import = true;
                continue;
            }

            // this is going to only be triggered after the import
            // block is filled
            // only one empty line should appear after the import
            if after_import {
                if l.trim().is_empty() {
                    continue;
                }
                after_import = false;
            }
            new_size += writer.write(format!("{}\n", l).as_bytes())?;
        }
        Ok(new_size)
    }
}

fn write_bucket<W: Write>(mut writer: W, imports: &[Import]) -> std::io::Result<usize> {
    let mut written: usize = 0;
    for i in imports {
        written += writer.write(format!("\t{}\n", i).as_bytes())?;
    }

    Ok(written)
}
