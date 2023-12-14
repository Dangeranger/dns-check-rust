# Rust Concurrent DNS Record Resolver

This is a command-line application written in Rust that resolves domain name records concurrently.

## Features

- Concurrent resolution of domain name records
- Support for multiple record types (A, AAAA, CNAME, etc.)
- Simple and intuitive command-line interface

## Pre-requisites

To use this application, you need to have Rust installed on your system. If you don't have Rust installed, you can get it from the official Rust website: [https://www.rust-lang.org/](https://www.rust-lang.org/)

## Installation

Once you have Rust installed, you can clone this repository and build the application using the following commands:

```sh
# Clone the repository
git clone ${REPO_URL}

# Change into the repository directory
cd dns-check-rust

# Build the application
cargo build
```

## Usage

To use the application, you can run it using the `cargo run` command or install it using the `cargo install` command and then run it using the `dns-check-rust` command.

```sh
# Check the application usage
cargo run -- --help

# Run the application
cargo run -- google.com A

# Install the application
cargo install --path .

# Run the installed application
dns-check-rust google.com A
```

## Contributing

Contributions are encouraged! Feel free to open an issue or submit a pull request if you have a bug fix, enhancement, or a new feature in mind.

Please note that this project is released with a [Contributor Code of Conduct](CODE_OF_CONDUCT.md). By participating in this project you agree to abide by its terms.****

## License

This project is licensed under the MPL License. See the [LICENSE](LICENSE.txt) file for more details.
