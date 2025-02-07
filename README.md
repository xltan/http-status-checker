# Rust CLI Application

This is a simple command-line interface (CLI) application written in Rust that accepts a URL as input and returns the HTTP status code of the response.

## Features

- Accepts a URL as a command-line argument.
- Makes an HTTP request to the provided URL.
- Returns the HTTP status code.

## Prerequisites

- Rust installed on your machine. You can install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

## Building the Application

To build the application, navigate to the project directory and run:

```
cargo build --release
```

This will compile the application in release mode, producing an optimized binary.

## Running the Application

After building the application, you can run it by providing a URL as an argument:

```
./target/release/http-status-checker <URL>
```

Replace `<URL>` with the actual URL you want to check.

## Example

```
./target/release/http-status-checker https://www.example.com
```

This command will output the HTTP status code for the specified URL.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.
