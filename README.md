# Data format Utils
![status](https://img.shields.io/badge/status-WIP-red)
[![crates.io](https://img.shields.io/crates/v/df-utils)](https://crates.io/crates/df-utils)
[![Downloads](https://img.shields.io/crates/d/df-utils)](https://crates.io/crates/df-utils)
[![Downloads (latest)](https://img.shields.io/crates/dv/df-utils)](https://crates.io/crates/df-utils)
[![License](https://img.shields.io/crates/l/df-utils)](https://github.com/watcol/df-utils/blob/main/LICENSE)

Simple CLI Parser for Data Formats

## Installation
Requires `cargo`
### Stable Version *(coming soon..)*
### Development Version
```
$ cargo install --git https://github.com/watcol/df-utils --branch main
```

## Usage
### As command line utilities
This repository contains these commands.

- `jprs` (JSON Parser)
- `jgen` (JSON Generator)
- `jfmt` (JSON Formatter)

Type `<cmd> -h` to see help for each command.

### As a Rust crate
Add this to your `Cargo.toml`:
```
[dependencies]
df-utils = { version = "comming soon...", default-features = false } # Stable Version
df-utils = { git = "https://github.com/watcol/df-utils", branch = "main", default-features = false } # Development Version
```

## Author
- ![watcol](https://raw.githubusercontent.com/watcol/icons/main/32/normal.png) Watcol <<potfman@gmail.com>>

## License
This program is licensed under the MIT license.

See [LICENSE](https://github.com/watcol/df-utils/blob/main/LICENSE) for details.
