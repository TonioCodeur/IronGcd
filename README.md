# iron-gcd

A Rust web server that calculates the **GCD** (Greatest Common Divisor) of two integers, built with the [Iron](https://github.com/iron/iron) framework.

## How it works

- `GET /` — displays an HTML form to enter two numbers
- `POST /gcd` — computes and returns the GCD of the submitted numbers
- `GET /*` — returns a 404 page for any other route

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024, via `rustup`)

Verify installation:

```bash
rustc --version
cargo --version
```

## Installation

Clone the repository and navigate to the directory:

```bash
git clone <repo-url>
cd iron-gcd
```

Download dependencies:

```bash
cargo build
```

## Running the server

```bash
cargo run
```

The server starts on **http://localhost:3000**.

Open this URL in a browser, enter two positive integers, and click **Compute GCD**.

## Dependencies

| Crate         | Purpose                               |
|---------------|---------------------------------------|
| `iron`        | HTTP framework                        |
| `router`      | Request routing                       |
| `urlencoded`  | Form data parsing                     |
| `mime`        | Response MIME type handling           |

