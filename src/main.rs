use clap::Parser;
use fgoi::run;

/// An S1 go imports sorting solution
#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
struct Options {
    /// Packages that need to be sorted separately from
    /// all the rest
    #[clap(short, long, use_value_delimiter = true, value_delimiter = ',')]
    package: Vec<String>,

    #[clap()]
    files: Vec<String>,
}

fn main() {
    let o = Options::parse();

    if let Err(e) = run(o.package, o.files) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
