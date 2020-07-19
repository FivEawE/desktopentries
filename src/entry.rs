use core::fmt;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

pub struct Entry {
    entries: HashMap<String, String>,
}

impl Entry {
    pub fn new(contents: &String) -> Result<Entry, ParseEntryError> {
        let mut entries = HashMap::new();

        for (index, line) in contents.lines().enumerate() {
            if line == "[Desktop Entry]" {
                continue;
            } else if line.starts_with('#') {
                continue;
            } else if line.trim() == "" {
                continue;
            } else if line.starts_with('[') {
                break;
            } else {
                let split: Vec<&str> = line.splitn(2, '=').collect();
                if split.len() < 2 {
                    return Err(ParseEntryError::new(format!(
                        "Could not extract value from line {}: {}",
                        index, line
                    )));
                }
                let key = split[0].trim();
                let value = split[1].trim();
                entries.insert(String::from(key), String::from(value));
            }
        }
        Ok(Entry { entries })
    }

    pub fn get_entries(&self) -> &HashMap<String, String> {
        &self.entries
    }

    #[cfg(test)]
    pub fn from_entries(entries: HashMap<String, String>) -> Entry {
        Entry { entries }
    }
}

#[derive(Debug)]
pub struct ParseEntryError {
    message: String,
}

impl ParseEntryError {
    fn new(message: String) -> ParseEntryError {
        ParseEntryError { message }
    }
}

impl Error for ParseEntryError {}

impl Display for ParseEntryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::error::Error;
    use std::io::{Seek, SeekFrom, Write};

    #[test]
    fn test_simple_file() -> Result<(), Box<dyn Error>> {
        let contents = String::from(
            "[Desktop Entry]
Name=My testing entry
Comment=Just a comment with another = for testing
GenericName=Test
Exec=/usr/bin/foo

Icon = bar
Type=Application
# Just a comment passing by
StartupNotify=false
Categories=Utility;Development;
MimeType=text/plain;

Actions=new-action;

# And another one
Keywords=test;

[Desktop Action test]
Name=New Empty Window
Exec=/usr/bin/foo -bar",
        );

        let entry = Entry::new(&contents)?;
        let entries = entry.get_entries();

        assert_eq!(entries.get("Name"), Some(&String::from("My testing entry")));
        assert_eq!(entries.get("Icon"), Some(&String::from("bar")));
        assert_eq!(
            entries.get("Comment"),
            Some(&String::from("Just a comment with another = for testing"))
        );
        assert_eq!(entries.get("Keywords"), Some(&String::from("test;")));
        Ok(())
    }

    #[test]
    fn test_invalid_file() -> Result<(), Box<dyn Error>> {
        let contents = String::from(
            "[Desktop Entry]
I am just an invalid entry",
        );

        let entry = Entry::new(&contents);

        assert!(entry
            .err()
            .unwrap()
            .message
            .contains("Could not extract value from line"));
        Ok(())
    }
}
