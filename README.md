# desktopentries

A simple tool for displaying and querying desktop entries

## Features

* A simple way to query and display desktop entries
* Supports localized keys (f.e. Name, Comment, etc.)
* Supports RegEx

## Installation

You can install the tool via `cargo`:
```
cargo install desktopentries
```

## Usage

To display all desktop entries, run `desktopentries` with no arguments:
```
desktopentries
```

To display all desktop entries whose `Type` is `Application`, run `desktopentries` with the `a` flag:
```
desktopentries -a
```

To display all desktop entries whose name is like `foo` case insensitively, contains `foo` and `bar` as keywords, are
terminal applications and are not hidden, run `desktopentries` as:
```
desktopentries -tH -n "(?i)foo" -k foo bar
```

To list all possible flags and options, run `desktopentries` with the `help` option:
```
desktopentries --help
```

## Notes

This tool tries to follow the Desktop Entry Specification. It looks for the entries in directories specified by
`$XDG_DATA_DIRS` environment variable. If the variable is not present, the tool looks for the entries in
`/usr/share/applications/` directory.

If multiple entry files with the same ID exist, the one located in the first entry directory will be chosen. If the
files exist in the same entry directory, the one which was detected first by Rust's ` std::fs::read_dir()`  will be
used (default behaviour is undefined by the specification).

The output of this tool consists of the paths to the desktop entry files with their contents which match the specified
flags and options. To extract particular lines from the output, you can use a tool such as `grep` and pipe the output of
this tool to it.

You can combine multiple flags and options, however, some of them are mutually exclusive as specified in the
specification (f.e. you cannot query for an entry which is of `Type` `Application` and contains a `URL` key).

All options take RegEx as values, so you can use them to query the entries as well. The tool uses The Rust Project
Developers's `regex` crate under the hood, so it does not support Lookarounds (Lookahead, Lookbehind).

The tool also supports localized keys as well. When the `-g` flag is provided, the tool looks for the value of
`$LC_MESSAGES` environment variable and uses it for searching. You can specify a different language using the `-G`
option, however, it does not check whether the value specified is a correct locale code.