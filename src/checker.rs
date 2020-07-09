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

        if conf.dbus {
            checks.push(Box::new(DBusActivatableCheck {}));
        }
        if conf.not_dbus {
            checks.push(Box::new(NotDBusActivatableCheck {}));
        }

        if conf.terminal {
            checks.push(Box::new(TerminalCheck {}));
        }
        if conf.not_terminal {
            checks.push(Box::new(NotTerminalCheck {}));
        }

        if conf.startup_notify {
            checks.push(Box::new(StartupNotifyCheck {}));
        }
        if conf.not_startup_notify {
            checks.push(Box::new(NotStartupNotifyCheck {}));
        }

        if conf.non_default_gpu {
            checks.push(Box::new(PrefersNonDefaultGPUCheck {}));
        }
        if conf.not_non_default_gpu {
            checks.push(Box::new(PrefersNonDefaultGPUCheck {}));
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

#[cfg(test)]
mod tests {}
