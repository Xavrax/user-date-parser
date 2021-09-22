# :warning: Work in progress! :warning:
# User Date Parser

![Rust](https://github.com/Xavrax/user-date-parser/actions/workflows/rust.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## What is it?

**User Date Parser** is a simple utility library that provides trait to parse user-friendly strings with
date identifications (*e.g. tomorrow*) to [`chrono::DateTime<Utc>`](https://docs.rs/chrono/) and vice versa.

Additionally, it includes some languages with simple implementations. [Here](#supported-build-in-parsers) is a list
of supported languages.

## Usage

### built-in functions
You have to import library in your `Cargo.toml` file then you can simply use exported functions like this:

> Note: you have to use `"en"` feature to use examples below

- #### user-friendly string to `chrono` 

```rust
use user_date_parser::friendly_parse;
use user_date_parser::parsers::En; // english language parser
use chrono::Utc;

fn main() {
    let user_friendly = "today";
    
    let date = friendly_parse::<En>(user_friendly);
    
    assert_eq!(Utc::today(), date);
}
```

- #### user-friendly string to `chrono` relative to other date

```rust
use user_date_parser::friendly_parse;
use user_date_parser::parsers::En; // english language parser
use chrono::Utc;

fn main() {
    let user_friendly = "today";
    let relative_date = Utc::today();
    
    let date = friendly_parse_relative::<En>(user_friendly, relative_date);
    
    assert_eq!(Utc::today(), date);
}
```

- #### `chrono` to user-friendly string

```rust
use user_date_parser::chrono_parse;
use user_date_parser::parsers::En; // english language parser
use chrono::Utc;

fn main() {
    let chrono_date = Utc::today();
    
    let date = chrono_parse::<En>(chrono_date);
    
    assert_eq!("today".into(), date);
}
```

- #### user-friendly string to `chrono`

```rust
use user_date_parser::chrono_parse_relative;
use user_date_parser::parsers::En; // english language parser
use chrono::Utc;

fn main() {
    let chrono_date = Utc::today();
    let relative_date = Utc::today();

    let date = chrono_parse_relative::<En>(chrono_date, relative_date);

    assert_eq!("today".into(), date);
}
```

### trait `UserDateParser`

You can implement your own parser using `UserDateParser` trait:

```rust
use user_date_parser::UserDateParser;

struct MyParser;

impl UserDateParser for MyParser {
    fn to_chrono(
        &self,
        friendly_string: Into<Str>,
        relative_date: Utc
    ) -> Result<Utc, UserDateParseError>
        where
            S: Into<Str> {
        // some implementation
    }

    fn to_user_friendly(
        &self,
        chrono_date: Utc,
        relative_date: Utc
    ) -> Result<Utc, UserDateParseError>
        where
            S: Into<Str> {
        // some implementation
    }
}

```

That's it!

## Supported build-in parsers

You can operate on supported build-in parsers by adding following features:

- `en` - english language
- `pl` - polish language
- `full` - all mentioned above

You can find more information about each parser [here](features/languages). This directory contains
gherkin files with language's features explanation.