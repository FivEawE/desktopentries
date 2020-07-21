use regex::Regex;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "desktopentries",
    about = "A simple tool for displaying and querying desktop entries",
    after_help = "This tool tries to follow the Desktop Entry Specification. It looks for the \
    entries in directories specified by $XDG_DATA_DIRS environment variable. If the variable is \
    not present, the tool looks for the entries in /usr/share/applications/ directory.\n\n\
    If multiple entry files with the same ID exist, the one located in the first entry directory \
    will be chosen. If the files exist in the same entry directory, the one which was detected \
    first by Rust's std::fs::read_dir() will be used (default behaviour is undefined by the \
    specification).\n\n\
    The output of this tool consists of the paths to the desktop entry files with their contents \
    which match the specified flags and options. To extract particular lines from the output, you \
    can use a tool such as grep and pipe the output of this tool to it.\n\n\
    You can combine multiple flags and options, however, some of them are mutually exclusive as \
    specified in the specification (f.e. you cannot query for an entry which is of Type \
    Application and contains a URL key).\n\n\
    All options take RegEx as values, so you can use them to query the entries as well. The tool \
    uses The Rust Project Developers's regex crate under the hood, so it does not support \
    Lookarounds (Lookahead, Lookbehind).\n\n\
    The tool also supports localized keys as well. When the -g flag is provided, the tool \
    looks for the value of $LC_MESSAGES environment variable and uses it for searching. You can \
    specify a different language using the -G option, however, it does not check whether the value \
    specified is a correct locale code."
)]
pub struct Configuration {
    #[structopt(
        short = "g",
        long = "localized",
        name = "Localized",
        overrides_with = "Language",
        help = "Sets the search to be localized according to host's locale"
    )]
    pub localized: bool,
    #[structopt(
        short = "G",
        long = "language",
        name = "Language",
        help = "Sets the search to be localized according to specified locale/language"
    )]
    pub lang: Option<String>,

    #[structopt(
        short = "a",
        long = "application",
        name = "Application",
        conflicts_with_all = &["Not Application", "Link", "Directory", "URL"],
        help = "Shows only entries where Type is Application",
    display_order = 1
    )]
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
        ],
        help = "Shows only entries where Type is Link",
    display_order = 2
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
        ],
        help = "Shows only entries where Type is Directory",
    display_order = 3
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
        ],
        help = "Shows only entries where Type is not Application",
    display_order = 4
    )]
    pub not_application: bool,
    #[structopt(
        short = "L",
        long = "not-link",
        name = "Not Link",
        conflicts_with = "URL",
        help = "Shows only entries where Type is not Link",
        display_order = 5
    )]
    pub not_link: bool,
    #[structopt(
        short = "D",
        long = "not-directory",
        name = "Not Directory",
        help = "Shows only entries where Type is not Directory",
        display_order = 6
    )]
    pub not_directory: bool,

    #[structopt(
        short = "v",
        long = "version",
        name = "Version",
        help = "Shows only entries where Version matches specified value",
        display_order = 7
    )]
    pub version: Option<Regex>,

    #[structopt(
        short = "n",
        long = "name",
        name = "Name",
        help = "Shows only entries where Name matches specified value",
        display_order = 8
    )]
    pub name: Option<Regex>,

    #[structopt(
        short = "N",
        long = "generic-name",
        name = "GenericName",
        help = "Shows only entries where GenericName matches specified value",
        display_order = 9
    )]
    pub generic_name: Option<Regex>,

    #[structopt(
        short = "y",
        long = "no-display",
        name = "NoDisplay",
        conflicts_with = "Not NoDisplay",
        help = "Shows only entries where NoDisplay is set to true",
        display_order = 10
    )]
    pub no_display: bool,
    #[structopt(
        short = "Y",
        long = "not-no-display",
        name = "Not NoDisplay",
        conflicts_with = "NoDisplay",
        help = "Shows only entries where NoDisplay is set to false or missing",
        display_order = 11
    )]
    pub not_no_display: bool,

    #[structopt(
        short = "c",
        long = "comment",
        name = "Comment",
        help = "Shows only entries where Comment matches specified value",
        display_order = 12
    )]
    pub comment: Option<Regex>,

    #[structopt(
        short = "i",
        long = "icon",
        name = "Icon",
        help = "Shows only entries where Icon matches specified value",
        display_order = 13
    )]
    pub icon: Option<Regex>,

    #[structopt(
        short = "h",
        long = "hidden",
        name = "Hidden",
        conflicts_with = "Not Hidden",
        help = "Shows only entries where Hidden is set to true",
        display_order = 14
    )]
    pub hidden: bool,
    #[structopt(
        short = "H",
        long = "not-hidden",
        name = "Not Hidden",
        help = "Shows only entries where Hidden is set to false or missing",
        display_order = 15
    )]
    pub not_hidden: bool,

    #[structopt(
        short = "o",
        long = "only-show-in",
        name = "OnlyShowIn",
        help = "Shows only entries where OnlyShowIn matches specified, space separated values",
        display_order = 16
    )]
    pub only_show_in: Option<Vec<Regex>>,
    #[structopt(
        short = "O",
        long = "not-show-in",
        name = "NotShowIn",
        help = "Shows only entries where NotShowIn matches specified, space separated values",
        display_order = 17
    )]
    pub not_show_in: Option<Vec<Regex>>,

    #[structopt(
        short = "b",
        long = "dbus-activatable",
        name = "DBusActivatable",
        conflicts_with = "Not DBusActivatable",
        help = "Shows only entries where DBusActivatable is set to true",
        display_order = 18
    )]
    pub dbus: bool,
    #[structopt(
        short = "B",
        long = "not-dbus-activatable",
        name = "Not DBusActivatable",
        help = "Shows only entries where DBusActivatable is set to false or empty",
        display_order = 19
    )]
    pub not_dbus: bool,

    #[structopt(
        short = "X",
        long = "try-exec",
        name = "TryExec",
        help = "Shows only entries where TryExec matches specified value",
        display_order = 20
    )]
    pub try_exec: Option<Regex>,
    #[structopt(
        short = "x",
        long = "exec",
        name = "Exec",
        help = "Shows only entries where Exec matches specified value",
        display_order = 21
    )]
    pub exec: Option<Regex>,

    #[structopt(
        short = "p",
        long = "path",
        name = "Path",
        help = "Shows only entries where Path matches specified value",
        display_order = 22
    )]
    pub path: Option<Regex>,

    #[structopt(
        short = "t",
        long = "terminal",
        name = "Terminal",
        conflicts_with = "Not Terminal",
        help = "Shows only entries where Terminal is set to true",
        display_order = 23
    )]
    pub terminal: bool,
    #[structopt(
        short = "T",
        long = "not-terminal",
        name = "Not Terminal",
        help = "Shows only entries where Terminal is set to false or missing",
        display_order = 24
    )]
    pub not_terminal: bool,

    #[structopt(
        short = "e",
        long = "actions",
        name = "Actions",
        help = "Shows only entries where Actions matches specified, space separated values",
        display_order = 25
    )]
    pub actions: Option<Vec<Regex>>,

    #[structopt(
        short = "m",
        long = "mime-type",
        name = "MimeType",
        help = "Shows only entries where MimeType matches specified, space separated values",
        display_order = 26
    )]
    pub mime_type: Option<Vec<Regex>>,

    #[structopt(
        short = "C",
        long = "categories",
        name = "Categories",
        help = "Shows only entries where Categories matches specified, space separated values",
        display_order = 27
    )]
    pub categories: Option<Vec<Regex>>,

    #[structopt(
        short = "I",
        long = "implements",
        name = "Implements",
        help = "Shows only entries where Implements matches specified, space separated values",
        display_order = 28
    )]
    pub implements: Option<Vec<Regex>>,

    #[structopt(
        short = "k",
        long = "keywords",
        name = "Keywords",
        help = "Shows only entries where Keywords matches specified, space separated values",
        display_order = 29
    )]
    pub keywords: Option<Vec<Regex>>,

    #[structopt(
        short = "s",
        long = "startup-notify",
        name = "StartupNotify",
        conflicts_with = "Not StartupNotify",
        help = "Shows only entries where StartupNotify is set to true",
        display_order = 30
    )]
    pub startup_notify: bool,
    #[structopt(
        short = "S",
        long = "not-startup-notify",
        name = "Not StartupNotify",
        help = "Shows only entries where StartupNotify is set to false or missing",
        display_order = 31
    )]
    pub not_startup_notify: bool,

    #[structopt(
        short = "w",
        long = "startup-wm-class",
        name = "StartupWMClass",
        help = "Shows only entries where StartupWMClass matches specified value",
        display_order = 32
    )]
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
        ],
        help = "Shows only entries where URL matches specified value",
        display_order = 33
    )]
    pub url: Option<Regex>,

    #[structopt(
        short = "z",
        long = "prefers-non-default-gpu",
        name = "PrefersNonDefaultGPU",
        conflicts_with = "Not PrefersNonDefaultGPU",
        help = "Shows only entries where PrefersNonDefaultGPU is set to true",
        display_order = 34
    )]
    pub non_default_gpu: bool,
    #[structopt(
        short = "Z",
        long = "not-prefers-non-default-gpu",
        name = "Not PrefersNonDefaultGPU",
        help = "Shows only entries where PrefersNonDefaultGPU is set to false or missing",
        display_order = 35
    )]
    pub not_non_default_gpu: bool,
}
