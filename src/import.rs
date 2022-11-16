use std::fmt;

#[derive(Clone)]
pub struct Import {
    pub name: Option<String>,
    pub url: String,
}

impl fmt::Display for Import {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(n) = &self.name {
            return write!(f, "{} \"{}\"", n, self.url);
        }
        write!(f, "\"{}\"", self.url)
    }
}
