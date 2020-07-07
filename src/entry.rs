use core::fmt;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Entry {
    entries: HashMap<String, String>,
}

impl Entry {
    pub fn new(file: File) -> Result<Entry, Box<dyn Error>> {
        let reader = BufReader::new(file);
        let mut entries = HashMap::new();

        for (index, line) in reader.lines().enumerate() {
            if let Ok(line) = line {
                if &line == "[Desktop Entry]" {
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
                        return Err(Box::new(ParseEntryError::new(format!(
                            "Could not extract value from line {}: {}",
                            index, line
                        ))));
                    }
                    let key = split[0].trim();
                    let value = split[1].trim();
                    entries.insert(String::from(key), String::from(value));
                }
            }
        }
        Ok(Entry { entries })
    }

    pub fn get_entries(&self) -> &HashMap<String, String> {
        &self.entries
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
    use crate::entry::{Entry, ParseEntryError};
    use std::error::Error;
    use std::io::{Seek, SeekFrom, Write};
    use tempfile;

    #[test]
    fn test_simple_file() -> Result<(), Box<dyn Error>> {
        let mut file = tempfile::tempfile()?;
        writeln!(
            file,
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
Exec=/usr/bin/foo -bar
"
        )?;
        file.seek(SeekFrom::Start(0))?;

        let entry = Entry::new(file)?;
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
        let mut file = tempfile::tempfile()?;
        writeln!(
            file,
            "[Desktop Entry]
I am just an invalid entry
"
        )?;
        file.seek(SeekFrom::Start(0))?;

        let entry = Entry::new(file);

        assert!(entry
            .err()
            .unwrap()
            .downcast::<ParseEntryError>()
            .unwrap()
            .message
            .contains("Could not extract value from line"));
        Ok(())
    }
}
