# RabbitSay

[![Rust CI](https://github.com/Cbeck527/rabbitsay/actions/workflows/ci.yaml/badge.svg)](https://github.com/Cbeck527/rabbitsay/actions/workflows/rust.yaml)
[![Release](https://github.com/Cbeck527/rabbitsay/actions/workflows/release.yaml/badge.svg)](https://github.com/Cbeck527/rabbitsay/actions/workflows/release.yaml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Say (possibly mean) things with a cute rabbit.

## Installation

```bash
# From source
cargo install rabbitsay

# From source
cargo install --path .
```

Or download from the [releases](https://github.com/cbeck527/rabbitsay/releases) page.

## Usage

```
$ rabbitsay "Hello, world!"
┌─────────────────┐
│  Hello, world!  │
└─────────────────┘
 (\__/) ||
 (•ㅅ•) ||
 / 　 づ
```

### CLI Options

```
Usage: rabbitsay [OPTIONS] <MESSAGE>

Arguments:
  <MESSAGE>  Message on the rabbit's sign

Options:
  -p, --padding <PADDING>      Padding around the message [default: 4]
  -w, --max-width <MAX_WIDTH>  Maximum width of the sign [default: 16]
  -h, --help                   Print help
  -V, --version                Print version
```

## As a Library

Add to your `Cargo.toml`:
```toml
[dependencies]
rabbitsay = { git = "https://github.com/Cbeck527/rabbitsay" }
```

Use in your code:
```rust
use rabbitsay::say;

fn main() {
    let output = say("Hello from Rust!");
    println!("{}", output);
}
```
