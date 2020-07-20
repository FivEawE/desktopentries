# desktopentries

A simple tool for displaying and querying desktop entries

## Features

* A simple way to query and display desktop entries
* Supports localized keys (f.e. Name, Comment, etc.)
* Supports RegEx

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