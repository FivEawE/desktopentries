use regex::Regex;
use std::collections::HashMap;
use std::env;

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
        let mut language_strings = Vec::with_capacity(0);

        if conf.localized {
            if let Some(lang) = env::var("LC_MESSAGES").ok() {
                language_strings = prepare_language_strings(lang);
            }
        }

        if let Some(lang) = conf.lang {
            language_strings = prepare_language_strings(lang);
        }

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
            checks.push(Box::new(VersionCheck { regex }));
        }

        if let Some(regex) = conf.name {
            checks.push(Box::new(NameCheck::new(regex, &language_strings)));
        }

        if let Some(regex) = conf.generic_name {
            checks.push(Box::new(GenericNameCheck::new(regex, &language_strings)));
        }

        if conf.no_display {
            checks.push(Box::new(NoDisplayCheck {}));
        }
        if conf.not_no_display {
            checks.push(Box::new(NotNoDisplayCheck {}));
        }

        if let Some(regex) = conf.comment {
            checks.push(Box::new(CommentCheck::new(regex, &language_strings)));
        }

        if let Some(regex) = conf.icon {
            checks.push(Box::new(IconCheck { regex }))
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

        if let Some(regex_list) = conf.keywords {
            checks.push(Box::new(KeywordsCheck::new(regex_list, &language_strings)));
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
        check_string_entry("Version", &self.regex, entries)
    }
}

struct NameCheck {
    regex: Regex,
    localized_keys: Vec<String>,
}
impl NameCheck {
    fn new(regex: Regex, language_strings: &Vec<String>) -> NameCheck {
        let localized_keys = create_localized_keys("Name", language_strings);
        NameCheck {
            regex,
            localized_keys,
        }
    }
}
impl Check for NameCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        check_localized_entry(&self.regex, &self.localized_keys, entries)
    }
}

struct GenericNameCheck {
    regex: Regex,
    localized_keys: Vec<String>,
}
impl GenericNameCheck {
    fn new(regex: Regex, language_strings: &Vec<String>) -> GenericNameCheck {
        let localized_keys = create_localized_keys("GenericName", language_strings);
        GenericNameCheck {
            regex,
            localized_keys,
        }
    }
}
impl Check for GenericNameCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        check_localized_entry(&self.regex, &self.localized_keys, entries)
    }
}

struct NoDisplayCheck {}
impl Check for NoDisplayCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        check_entry_true("NoDisplay", entries)
    }
}
struct NotNoDisplayCheck {}
impl Check for NotNoDisplayCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        check_entry_false("NoDisplay", entries)
    }
}

struct CommentCheck {
    regex: Regex,
    localized_keys: Vec<String>,
}
impl CommentCheck {
    fn new(regex: Regex, language_strings: &Vec<String>) -> CommentCheck {
        let localized_keys = create_localized_keys("Comment", language_strings);
        CommentCheck {
            regex,
            localized_keys,
        }
    }
}
impl Check for CommentCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        check_localized_entry(&self.regex, &self.localized_keys, entries)
    }
}

struct IconCheck {
    regex: Regex,
}
impl Check for IconCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        check_string_entry("Icon", &self.regex, entries)
    }
}

struct HiddenCheck {}
impl Check for HiddenCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        check_entry_true("Hidden", entries)
    }
}
struct NotHiddenCheck {}
impl Check for NotHiddenCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        check_entry_false("Hidden", entries)
    }
}

struct OnlyShowInCheck {
    regex_list: Vec<Regex>,
}
impl Check for OnlyShowInCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        check_multi_string_entry("OnlyShowIn", &self.regex_list, entries)
    }
}
struct NotShowInCheck {
    regex_list: Vec<Regex>,
}
impl Check for NotShowInCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        check_multi_string_entry("NotShowIn", &self.regex_list, entries)
    }
}

struct DBusActivatableCheck {}
impl Check for DBusActivatableCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        check_entry_true("DBusActivatable", entries)
    }
}
struct NotDBusActivatableCheck {}
impl Check for NotDBusActivatableCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        check_entry_false("DBusActivatable", entries)
    }
}

struct TryExecCheck {
    regex: Regex,
}
impl Check for TryExecCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        check_string_entry("TryExec", &self.regex, entries)
    }
}

struct ExecCheck {
    regex: Regex,
}
impl Check for ExecCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        check_string_entry("Exec", &self.regex, entries)
    }
}

struct PathCheck {
    regex: Regex,
}
impl Check for PathCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        check_string_entry("Path", &self.regex, entries)
    }
}

struct TerminalCheck {}
impl Check for TerminalCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        check_entry_true("Terminal", entries)
    }
}
struct NotTerminalCheck {}
impl Check for NotTerminalCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        check_entry_false("Terminal", entries)
    }
}

struct ActionsCheck {
    regex_list: Vec<Regex>,
}
impl Check for ActionsCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        check_multi_string_entry("Actions", &self.regex_list, entries)
    }
}

struct MimeTypeCheck {
    regex_list: Vec<Regex>,
}
impl Check for MimeTypeCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        check_multi_string_entry("MimeType", &self.regex_list, entries)
    }
}

struct CategoriesCheck {
    regex_list: Vec<Regex>,
}
impl Check for CategoriesCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        check_multi_string_entry("Categories", &self.regex_list, entries)
    }
}

struct ImplementsCheck {
    regex_list: Vec<Regex>,
}
impl Check for ImplementsCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        check_multi_string_entry("Implements", &self.regex_list, entries)
    }
}

struct KeywordsCheck {
    regex_list: Vec<Regex>,
    localized_keys: Vec<String>,
}
impl KeywordsCheck {
    fn new(regex_list: Vec<Regex>, language_strings: &Vec<String>) -> KeywordsCheck {
        let localized_keys = create_localized_keys("Keywords", language_strings);
        KeywordsCheck {
            regex_list,
            localized_keys,
        }
    }
}
impl Check for KeywordsCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        for key in &self.localized_keys {
            if check_multi_string_entry(key, &self.regex_list, entries) {
                return true;
            }
        }
        false
    }
}

struct StartupNotifyCheck {}
impl Check for StartupNotifyCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        check_entry_true("StartupNotify", entries)
    }
}
struct NotStartupNotifyCheck {}
impl Check for NotStartupNotifyCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        check_entry_false("StartupNotify", entries)
    }
}

struct StartupWMClassCheck {
    regex: Regex,
}
impl Check for StartupWMClassCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        check_string_entry("StartupWMClass", &self.regex, entries)
    }
}

struct URLCheck {
    regex: Regex,
}
impl Check for URLCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        check_string_entry("URL", &self.regex, entries)
    }
}

struct PrefersNonDefaultGPUCheck {}
impl Check for PrefersNonDefaultGPUCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        check_entry_true("PrefersNonDefaultGPU", entries)
    }
}
struct NotPrefersNonDefaultGPUCheck {}
impl Check for NotPrefersNonDefaultGPUCheck {
    fn check(&self, entries: &HashMap<String, String>) -> bool {
        check_entry_false("PrefersNonDefaultGPU", entries)
    }
}

fn check_entry_true(key: &str, entries: &HashMap<String, String>) -> bool {
    let entry = entries.get(key);
    match entry {
        Some(value) => value == TRUE,
        None => false,
    }
}

fn check_entry_false(key: &str, entries: &HashMap<String, String>) -> bool {
    let entry = entries.get(key);
    match entry {
        Some(value) => value == FALSE,
        None => true,
    }
}

fn check_string_entry(key: &str, regex: &Regex, entries: &HashMap<String, String>) -> bool {
    let entry = entries.get(key);
    match entry {
        Some(value) => regex.is_match(value),
        None => false,
    }
}

fn check_multi_string_entry(
    key: &str,
    regex_list: &Vec<Regex>,
    entries: &HashMap<String, String>,
) -> bool {
    let entry = entries.get(key);
    match entry {
        Some(value) => {
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
        None => false,
    }
}

fn check_localized_entry(
    regex: &Regex,
    localized_keys: &Vec<String>,
    entries: &HashMap<String, String>,
) -> bool {
    for key in localized_keys {
        let entry = entries.get(key);
        if let Some(value) = entry {
            return regex.is_match(value);
        }
    }
    false
}

fn prepare_language_strings(lang: String) -> Vec<String> {
    let mut language_strings: Vec<String> = Vec::with_capacity(4);

    let level1 = Regex::new(r"(.+)_([^.]+)(?:\..+)?@(.+)").unwrap();
    let level2 = Regex::new(r"(.+)_([^.@]+)(?:[.@].+)?").unwrap();
    let level3 = Regex::new(r"([^_]+)(?:_.+)?@(.+)").unwrap();
    let level4 = Regex::new(r"([^_@]+)(?:[_@].+)?").unwrap();

    if let Some(capture) = level1.captures(&lang) {
        language_strings.push(format!("[{}_{}@{}]", &capture[1], &capture[2], &capture[3]));
    }
    if let Some(capture) = level2.captures(&lang) {
        language_strings.push(format!("[{}_{}]", &capture[1], &capture[2]));
    }
    if let Some(capture) = level3.captures(&lang) {
        language_strings.push(format!("[{}@{}]", &capture[1], &capture[2]));
    }
    if let Some(capture) = level4.captures(&lang) {
        language_strings.push(format!("[{}]", &capture[1]));
    }

    language_strings
}

fn create_localized_keys(prefix: &str, language_strings: &Vec<String>) -> Vec<String> {
    let mut localized_keys = Vec::with_capacity(language_strings.len() + 1);
    for string in language_strings {
        localized_keys.push(format!("{}{}", prefix, string));
    }
    localized_keys.push(String::from(prefix));

    localized_keys
}

#[cfg(test)]
mod tests {
    use super::Checker;
    use super::Configuration;
    use super::Entry;
    use std::collections::HashMap;
    use std::env;
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
            "-w",
            "(?i)NOTIFICATION",
        ]);

        let checker = Checker::new(conf);
        assert!(checker.check_entry(&entry));
    }

    #[test]
    fn test_link() {
        let mut entries = HashMap::new();
        entries.insert(String::from("Type"), String::from("Link"));
        entries.insert(String::from("URL"), String::from("https://fiveawe.com/"));

        let entry = Entry::from_entries(entries);
        let conf = Configuration::from_iter(&[APP_NAME, "-lAD", "-u", "https://fiveawe.com/"]);

        let checker = Checker::new(conf);
        assert!(checker.check_entry(&entry));
    }

    #[test]
    fn test_localized() {
        let mut entries = HashMap::new();
        entries.insert(String::from("Name[en_GB@Latn]"), String::from("FooBar"));
        entries.insert(String::from("GenericName[en_GB]"), String::from("Bar"));
        entries.insert(String::from("Comment[en]"), String::from("Just a test"));

        let entry = Entry::from_entries(entries);
        let conf =
            Configuration::from_iter(&[APP_NAME, "-g", "-n", "Foo", "-N", "Bar", "-c", "test"]);

        env::set_var("LC_MESSAGES", "en_GB.ASCII@Latn");
        let checker = Checker::new(conf);
        assert!(checker.check_entry(&entry));
    }

    #[test]
    fn test_localized2() {
        let mut entries = HashMap::new();
        entries.insert(String::from("Name[en@Latn]"), String::from("FooBar"));
        entries.insert(String::from("GenericName[en]"), String::from("Bar"));
        entries.insert(String::from("Comment"), String::from("Just a test"));

        let entry = Entry::from_entries(entries);
        let conf = Configuration::from_iter(&[
            APP_NAME, "-n", "Foo", "-N", "Bar", "-c", "test", "-G", "en@Latn",
        ]);

        let checker = Checker::new(conf);
        assert!(checker.check_entry(&entry));
    }

    #[test]
    fn test_localized_priority() {
        let mut entries = HashMap::new();
        entries.insert(String::from("Keywords[en_GB]"), String::from("Foo;Bar;"));
        entries.insert(
            String::from("Keywords[en@Latn]"),
            String::from("Three;Two;"),
        );

        let entry = Entry::from_entries(entries);
        let conf = Configuration::from_iter(&[APP_NAME, "-g", "-k", "Foo", "Bar"]);

        env::set_var("LC_MESSAGES", "en_GB.ASCII@Latn");
        let checker = Checker::new(conf);
        assert!(checker.check_entry(&entry));
    }

    #[test]
    fn test_mix() {
        let mut entries = HashMap::new();
        entries.insert(String::from("Version"), String::from("1.0.0"));
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
        entries.insert(String::from("Keywords[en_GB]"), String::from("Foo;Bar;"));
        entries.insert(
            String::from("Keywords[en@Latn]"),
            String::from("Three;Two;"),
        );
        entries.insert(String::from("Type"), String::from("Application"));
        entries.insert(String::from("NoDisplay"), String::from("true"));
        entries.insert(String::from("Hidden"), String::from("true"));
        entries.insert(String::from("DBusActivatable"), String::from("true"));
        entries.insert(String::from("Terminal"), String::from("false"));
        entries.insert(String::from("StartupNotify"), String::from("false"));

        let entry = Entry::from_entries(entries);
        let conf = Configuration::from_iter(&[
            APP_NAME,
            "-v",
            "1",
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
            "-w",
            "(?i)NOTIFICATION",
            "-aLDyhbTSZg",
            "-k",
            "Foo",
            "Bar",
        ]);

        env::set_var("LC_MESSAGES", "en_GB.ASCII@Latn");
        let checker = Checker::new(conf);
        assert!(checker.check_entry(&entry));
    }
}
