use regex::Regex;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "desktopentries",
    about = "An easy tool for displaying and querying desktop entries"
)]
pub struct Configuration {
    #[structopt(
        short = "a",
        long = "application",
        name = "Application",
        conflicts_with_all = &["Not Application", "Link", "Directory", "URL"])]
    pub application: bool,
    #[structopt(
        short = "l",
        long = "link",
        name = "Link",
        conflicts_with_all = &[
            "Not Link", "Directory", "TryExec", "Exec", "Path", "Terminal", "Not Terminal",
            "Actions", "MimeType", "Categories", "Implements", "Keywords", "StartupNotify",
            "Not StartupNotify", "StartupWMClass", "Not StartupWMClass", "PrefersNonDefaultGPU",
            "Not PrefersNonDefaultGPU"
        ]
    )]
    pub link: bool,
    #[structopt(
        short = "d",
        long = "directory",
        name = "Directory",
        conflicts_with_all = &[
            "Not Directory", "URL", "TryExec", "Exec", "Path", "Terminal", "Not Terminal",
            "Actions", "MimeType", "Categories", "Implements", "Keywords", "StartupNotify",
            "Not StartupNotify", "StartupWMClass", "Not StartupWMClass", "PrefersNonDefaultGPU",
            "Not PrefersNonDefaultGPU"
        ]
    )]
    pub directory: bool,

    #[structopt(
        short = "A",
        long = "not-application",
        name = "Not Application",
        conflicts_with_all = &[
            "TryExec", "Exec", "Path", "Terminal", "Not Terminal", "Actions", "MimeType",
            "Categories", "Implements", "Keywords", "StartupNotify", "Not StartupNotify",
            "StartupWMClass", "Not StartupWMClass", "PrefersNonDefaultGPU",
            "Not PrefersNonDefaultGPU"
        ]
    )]
    pub not_application: bool,
    #[structopt(
        short = "L",
        long = "not-link",
        name = "Not Link",
        conflicts_with = "URL"
    )]
    pub not_link: bool,
    #[structopt(short = "D", long = "not-directory", name = "Not Directory")]
    pub not_directory: bool,

    #[structopt(short = "v", long = "version", name = "Version")]
    pub version: Option<Regex>,

    #[structopt(short = "n", long = "name", name = "Name")]
    pub name: Option<Regex>,

    #[structopt(short = "g", long = "generic-name", name = "GenericName")]
    pub generic_name: Option<Regex>,

    #[structopt(
        short = "y",
        long = "no-display",
        name = "NoDisplay",
        conflicts_with = "Not NoDisplay"
    )]
    pub no_display: bool,
    #[structopt(
        short = "Y",
        long = "not-no-display",
        name = "Not NoDisplay",
        conflicts_with = "NoDisplay"
    )]
    pub not_no_display: bool,

    #[structopt(short = "c", long = "comment", name = "Comment")]
    pub comment: Option<Regex>,

    #[structopt(short = "i", long = "icon", name = "Icon")]
    pub icon: Option<Regex>,

    #[structopt(
        short = "h",
        long = "hidden",
        name = "Hidden",
        conflicts_with = "Not Hidden"
    )]
    pub hidden: bool,
    #[structopt(short = "H", long = "not-hidden", name = "Not Hidden")]
    pub not_hidden: bool,

    #[structopt(short = "o", long = "only-show-in", name = "OnlyShowIn")]
    pub only_show_in: Option<Vec<Regex>>,
    #[structopt(short = "O", long = "not-show-in", name = "NotShowIn")]
    pub not_show_in: Option<Vec<Regex>>,

    #[structopt(
        short = "b",
        long = "dbus-activatable",
        name = "DBusActivatable",
        conflicts_with = "Not DBusActivatable"
    )]
    pub dbus: bool,
    #[structopt(
        short = "B",
        long = "not-dbus-activatable",
        name = "Not DBusActivatable"
    )]
    pub not_dbus: bool,

    #[structopt(short = "X", long = "try-exec", name = "TryExec")]
    pub try_exec: Option<Regex>,
    #[structopt(short = "x", long = "exec", name = "Exec")]
    pub exec: Option<Regex>,

    #[structopt(short = "p", long = "path", name = "Path")]
    pub path: Option<Regex>,

    #[structopt(
        short = "t",
        long = "terminal",
        name = "Terminal",
        conflicts_with = "Not Terminal"
    )]
    pub terminal: bool,
    #[structopt(short = "T", long = "not-terminal", name = "Not Terminal")]
    pub not_terminal: bool,

    #[structopt(short = "e", long = "actions", name = "Actions")]
    pub actions: Option<Vec<Regex>>,

    #[structopt(short = "m", long = "mime-type", name = "MimeType")]
    pub mime_type: Option<Vec<Regex>>,

    #[structopt(short = "C", long = "categories", name = "Categories")]
    pub categories: Option<Vec<Regex>>,

    #[structopt(short = "I", long = "implements", name = "Implements")]
    pub implements: Option<Vec<Regex>>,

    #[structopt(short = "k", long = "keywords", name = "Keywords")]
    pub keywords: Option<Vec<Regex>>,

    #[structopt(
        short = "s",
        long = "startup-notify",
        name = "StartupNotify",
        conflicts_with = "Not StartupNotify"
    )]
    pub startup_notify: bool,
    #[structopt(short = "S", long = "not-startup-notify", name = "Not StartupNotify")]
    pub not_startup_notify: bool,

    #[structopt(short = "w", long = "startup-wm-class", name = "StartupWMClass")]
    pub wm_class: Option<Regex>,

    #[structopt(
        short = "u",
        long = "url",
        name = "URL",
        conflicts_with_all = &[
            "TryExec", "Exec", "Path", "Terminal", "Not Terminal", "Actions", "MimeType",
            "Categories", "Implements", "Keywords", "StartupNotify", "Not StartupNotify",
            "StartupWMClass", "Not StartupWMClass", "PrefersNonDefaultGPU",
            "Not PrefersNonDefaultGPU"
        ]
    )]
    pub url: Option<Regex>,

    #[structopt(
        short = "z",
        long = "prefers-non-default-gpu",
        name = "PrefersNonDefaultGPU",
        conflicts_with = "Not PrefersNonDefaultGPU"
    )]
    pub non_default_gpu: bool,
    #[structopt(
        short = "Z",
        long = "not-prefers-non-default-gpu",
        name = "Not PrefersNonDefaultGPU"
    )]
    pub not_non_default_gpu: bool,
}
