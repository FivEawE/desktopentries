use regex::Regex;
use std::collections::HashMap;

use super::configuration::Configuration;
use super::entry::Entry;

static TRUE: &str = "true";
static FALSE: &str = "false";

pub struct Checker {
    checks: Vec<Box<dyn Check>>,
}

impl Checker {
    pub fn new(conf: Configuration) -> Checker {
        let mut checks: Vec<Box<dyn Check>> = Vec::new();

        if conf.application {
            checks.push(Box::new(ApplicationCheck {}));
        }
        if conf.link {
            checks.push(Box::new(LinkCheck {}));
        }
        if conf.directory {
            checks.push(Box::new(DirectoryCheck {}));
        }

        if conf.not_application {
            checks.push(Box::new(NotApplicationCheck {}));
        }
        if conf.not_link {
            checks.push(Box::new(NotLinkCheck {}));
        }
        if conf.not_directory {
            checks.push(Box::new(NotDirectoryCheck {}));
        }

        if let Some(regex) = conf.version {
            checks.push(Box::new(VersionCheck { regex }))
        }

        if let Some(regex) = conf.icon {
            checks.push(Box::new(IconCheck { regex }))
        }

        if conf.no_display {
            checks.push(Box::new(NoDisplayCheck {}));
        }
        if conf.not_no_display {
            checks.push(Box::new(NotNoDisplayCheck {}));
        }

        if conf.hidden {
            checks.push(Box::new(HiddenCheck {}));
        }
        if conf.not_hidden {
            checks.push(Box::new(NotHiddenCheck {}));
        }

        if let Some(regex_list) = conf.only_show_in {
            checks.push(Box::new(OnlyShowInCheck { regex_list }))
        }
        if let Some(regex_list) = conf.not_show_in {
            checks.push(Box::new(NotShowInCheck { regex_list }))
        }

        if conf.dbus {
            checks.push(Box::new(DBusActivatableCheck {}));
        }
        if conf.not_dbus {
            checks.push(Box::new(NotDBusActivatableCheck {}));
        }

        if let Some(regex) = conf.try_exec {
            checks.push(Box::new(TryExecCheck { regex }))
        }

        if let Some(regex) = conf.exec {
            checks.push(Box::new(ExecCheck { regex }))
        }

        if let Some(regex) = conf.path {
            checks.push(Box::new(PathCheck { regex }))
        }

        if conf.terminal {
            checks.push(Box::new(TerminalCheck {}));
        }
        if conf.not_terminal {
            checks.push(Box::new(NotTerminalCheck {}));
        }

        if let Some(regex_list) = conf.actions {
            checks.push(Box::new(ActionsCheck { regex_list }))
        }

        if let Some(regex_list) = conf.mime_type {
            checks.push(Box::new(MimeTypeCheck { regex_list }))
        }

        if let Some(regex_list) = conf.categories {
            checks.push(Box::new(CategoriesCheck { regex_list }))
        }

        if let Some(regex_list) = conf.implements {
            checks.push(Box::new(ImplementsCheck { regex_list }))
        }

        if conf.startup_notify {
            checks.push(Box::new(StartupNotifyCheck {}));
        }
        if conf.not_startup_notify {
            checks.push(Box::new(NotStartupNotifyCheck {}));
        }

        if let Some(regex) = conf.wm_class {
            checks.push(Box::new(StartupWMClassCheck { regex }))
        }

        if let Some(regex) = conf.url {
            checks.push(Box::new(URLCheck { regex }))
        }

        if conf.non_default_gpu {
            checks.push(Box::new(PrefersNonDefaultGPUCheck {}));
        }
        if conf.not_non_default_gpu {
            checks.push(Box::new(NotPrefersNonDefaultGPUCheck {}));
        }

        Checker { checks }
    }

    pub fn check_entry(&self, entry: &Entry) -> bool {
        for check in &self.checks {
            if !check.check(entry.get_entries()) {
                return false;
            }
        }
        true
    }
}

trait Check {
    fn check(&self, entries: &HashMap<String, String>) -> bool;
}

struct ApplicationCheck {}
impl Check for ApplicationCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("Type");
        match entry {
            Some(value) => value == "Application",
            None => false,
        }
    }
}
struct LinkCheck {}
impl Check for LinkCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("Type");
        match entry {
            Some(value) => value == "Link",
            None => false,
        }
    }
}
struct DirectoryCheck {}
impl Check for DirectoryCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("Type");
        match entry {
            Some(value) => value == "Directory",
            None => false,
        }
    }
}

struct NotApplicationCheck {}
impl Check for NotApplicationCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("Type");
        match entry {
            Some(value) => value != "Application",
            None => false,
        }
    }
}
struct NotLinkCheck {}
impl Check for NotLinkCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("Type");
        match entry {
            Some(value) => value != "Link",
            None => false,
        }
    }
}
struct NotDirectoryCheck {}
impl Check for NotDirectoryCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("Type");
        match entry {
            Some(value) => value != "Directory",
            None => false,
        }
    }
}

struct VersionCheck {
    regex: Regex,
}
impl Check for VersionCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("Version");
        match entry {
            Some(value) => self.regex.is_match(value),
            None => false,
        }
    }
}

struct NoDisplayCheck {}
impl Check for NoDisplayCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("NoDisplay");
        match entry {
            Some(value) => value == TRUE,
            None => false,
        }
    }
}
struct NotNoDisplayCheck {}
impl Check for NotNoDisplayCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("NoDisplay");
        match entry {
            Some(value) => value == FALSE,
            None => true,
        }
    }
}

struct IconCheck {
    regex: Regex,
}
impl Check for IconCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("Icon");
        match entry {
            Some(value) => self.regex.is_match(value),
            None => false,
        }
    }
}

struct HiddenCheck {}
impl Check for HiddenCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("Hidden");
        match entry {
            Some(value) => value == TRUE,
            None => false,
        }
    }
}
struct NotHiddenCheck {}
impl Check for NotHiddenCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("Hidden");
        match entry {
            Some(value) => value == FALSE,
            None => true,
        }
    }
}

struct OnlyShowInCheck {
    regex_list: Vec<Regex>,
}
impl Check for OnlyShowInCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("OnlyShowIn");
        match entry {
            Some(value) => check_multi_string_entry(value, &self.regex_list),
            None => true,
        }
    }
}
struct NotShowInCheck {
    regex_list: Vec<Regex>,
}
impl Check for NotShowInCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("NotShowIn");
        match entry {
            Some(value) => check_multi_string_entry(value, &self.regex_list),
            None => true,
        }
    }
}

struct DBusActivatableCheck {}
impl Check for DBusActivatableCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("DBusActivatable");
        match entry {
            Some(value) => value == TRUE,
            None => false,
        }
    }
}
struct NotDBusActivatableCheck {}
impl Check for NotDBusActivatableCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("DBusActivatable");
        match entry {
            Some(value) => value == FALSE,
            None => true,
        }
    }
}

struct TryExecCheck {
    regex: Regex,
}
impl Check for TryExecCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("TryExec");
        match entry {
            Some(value) => self.regex.is_match(value),
            None => false,
        }
    }
}

struct ExecCheck {
    regex: Regex,
}
impl Check for ExecCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("Exec");
        match entry {
            Some(value) => self.regex.is_match(value),
            None => false,
        }
    }
}

struct PathCheck {
    regex: Regex,
}
impl Check for PathCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("Path");
        match entry {
            Some(value) => self.regex.is_match(value),
            None => false,
        }
    }
}

struct TerminalCheck {}
impl Check for TerminalCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("Terminal");
        match entry {
            Some(value) => value == TRUE,
            None => false,
        }
    }
}
struct NotTerminalCheck {}
impl Check for NotTerminalCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("Terminal");
        match entry {
            Some(value) => value == FALSE,
            None => true,
        }
    }
}

struct ActionsCheck {
    regex_list: Vec<Regex>,
}
impl Check for ActionsCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("Actions");
        match entry {
            Some(value) => check_multi_string_entry(value, &self.regex_list),
            None => true,
        }
    }
}

struct MimeTypeCheck {
    regex_list: Vec<Regex>,
}
impl Check for MimeTypeCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("MimeType");
        match entry {
            Some(value) => check_multi_string_entry(value, &self.regex_list),
            None => true,
        }
    }
}

struct CategoriesCheck {
    regex_list: Vec<Regex>,
}
impl Check for CategoriesCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("Categories");
        match entry {
            Some(value) => check_multi_string_entry(value, &self.regex_list),
            None => true,
        }
    }
}

struct ImplementsCheck {
    regex_list: Vec<Regex>,
}
impl Check for ImplementsCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("Implements");
        match entry {
            Some(value) => check_multi_string_entry(value, &self.regex_list),
            None => true,
        }
    }
}

struct StartupNotifyCheck {}
impl Check for StartupNotifyCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("StartupNotify");
        match entry {
            Some(value) => value == TRUE,
            None => false,
        }
    }
}
struct NotStartupNotifyCheck {}
impl Check for NotStartupNotifyCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("StartupNotify");
        match entry {
            Some(value) => value == FALSE,
            None => true,
        }
    }
}

struct StartupWMClassCheck {
    regex: Regex,
}
impl Check for StartupWMClassCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("StartupWMClass");
        match entry {
            Some(value) => self.regex.is_match(value),
            None => false,
        }
    }
}

struct URLCheck {
    regex: Regex,
}
impl Check for URLCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("URL");
        match entry {
            Some(value) => self.regex.is_match(value),
            None => false,
        }
    }
}

struct PrefersNonDefaultGPUCheck {}
impl Check for PrefersNonDefaultGPUCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("PrefersNonDefaultGPU");
        match entry {
            Some(value) => value == TRUE,
            None => false,
        }
    }
}
struct NotPrefersNonDefaultGPUCheck {}
impl Check for NotPrefersNonDefaultGPUCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        let entry = entries.get("PrefersNonDefaultGPU");
        match entry {
            Some(value) => value == FALSE,
            None => true,
        }
    }
}

fn check_multi_string_entry(value: &String, regex_list: &Vec<Regex>) -> bool {
    let regex_count = regex_list.len();
    let mut count = 0;
    let values: Vec<&str> = value.split(';').collect();

    for regex in regex_list {
        if count == regex_count {
            return true;
        }

        for &string in &values {
            if regex.is_match(string) {
                count += 1;
                break;
            }
        }
    }
    count == regex_count
}

#[cfg(test)]
mod tests {
    use super::Checker;
    use super::Configuration;
    use super::Entry;
    use std::collections::HashMap;
    use structopt::StructOpt;

    static APP_NAME: &str = "desktopentries";

    #[test]
    fn test_trues() {
        let mut entries = HashMap::new();
        entries.insert(String::from("Type"), String::from("Application"));
        entries.insert(String::from("NoDisplay"), String::from("true"));
        entries.insert(String::from("Hidden"), String::from("true"));
        entries.insert(String::from("DBusActivatable"), String::from("true"));
        entries.insert(String::from("Terminal"), String::from("true"));
        entries.insert(String::from("StartupNotify"), String::from("true"));
        entries.insert(String::from("PrefersNonDefaultGPU"), String::from("true"));

        let entry = Entry::from_entries(entries);
        let conf = Configuration::from_iter(&[APP_NAME, "-aLDyhbtsz"]);

        let checker = Checker::new(conf);
        assert!(checker.check_entry(&entry));
    }

    #[test]
    fn test_falses() {
        let mut entries = HashMap::new();
        entries.insert(String::from("Type"), String::from("Application"));
        entries.insert(String::from("NoDisplay"), String::from("false"));
        entries.insert(String::from("Hidden"), String::from("false"));
        entries.insert(String::from("DBusActivatable"), String::from("false"));
        entries.insert(String::from("Terminal"), String::from("false"));
        entries.insert(String::from("StartupNotify"), String::from("false"));
        entries.insert(String::from("PrefersNonDefaultGPU"), String::from("false"));

        let entry = Entry::from_entries(entries);
        let conf = Configuration::from_iter(&[APP_NAME, "-LDYHBTSZ"]);

        let checker = Checker::new(conf);
        assert!(checker.check_entry(&entry));
    }

    #[test]
    fn test_empty_falses() {
        let mut entries = HashMap::new();
        entries.insert(String::from("Type"), String::from("Application"));

        let entry = Entry::from_entries(entries);
        let conf = Configuration::from_iter(&[APP_NAME, "-LDYHBTSZ"]);

        let checker = Checker::new(conf);
        assert!(checker.check_entry(&entry));
    }

    #[test]
    fn test_regex() {
        let mut entries = HashMap::new();
        entries.insert(String::from("Version"), String::from("1.0.1"));
        entries.insert(String::from("Icon"), String::from("fOoBaR1"));
        entries.insert(
            String::from("OnlyShowIn"),
            String::from("one;two;three;one;"),
        );
        entries.insert(String::from("NotShowIn"), String::from("four;five;six;"));
        entries.insert(String::from("TryExec"), String::from("foo"));
        entries.insert(String::from("Exec"), String::from("bar"));
        entries.insert(String::from("Path"), String::from("/foo/bar/abc/def"));
        entries.insert(
            String::from("Actions"),
            String::from("New Window;Hidden Window;"),
        );
        entries.insert(
            String::from("MimeType"),
            String::from("image/png;image/jpg;"),
        );
        entries.insert(String::from("Categories"), String::from("Audio;Video;"));
        entries.insert(String::from("Implements"), String::from("something;"));
        entries.insert(
            String::from("Keywords"),
            String::from("foo;bar;entry;desktop;"),
        );
        entries.insert(
            String::from("StartupWMClass"),
            String::from("Just a notification"),
        );

        let entry = Entry::from_entries(entries);
        let conf = Configuration::from_iter(&[
            APP_NAME,
            "-v",
            "1.0",
            "-i",
            "(?i)bar\\d",
            "-o",
            "three",
            "one",
            "-O",
            "five",
            "-X",
            "o",
            "-x",
            "[^foo]",
            "-p",
            "/abc/",
            "-e",
            "Window",
            "-m",
            "(?i)image",
            "-C",
            "dio",
            "deo",
            "-I",
            "thing",
            "-k",
            "desktop",
            "entry",
            "-w",
            "(?i)NOTIFICATION",
        ]);

        let checker = Checker::new(conf);
        assert!(checker.check_entry(&entry));
    }
}
