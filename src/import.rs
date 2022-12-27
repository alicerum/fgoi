use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl From<(&str, &str)> for Import {
    fn from((name, url): (&str, &str)) -> Self {
        Self {
            name: Some(name.to_string()),
            url: url.to_string(),
        }
    }
}

impl From<(Option<&str>, &str)> for Import {
    fn from((name, url): (Option<&str>, &str)) -> Self {
        Self {
            name: name.map(str::to_string),
            url: url.to_string(),
        }
    }
}
