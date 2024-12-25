# File Extension Fixer Rust

A tool written in Rust that looks over every photo and video in a directory and corrects the file extension based off of the magic number of the file.

This is the Rust equivalent of [File Extension Fixer JS](https://github.com/SmollClover/File-Extension-Fixer-JS).

---

## Cache

File Extension Fixer Rust uses no cache due to its speed but I may include a caching feature in the future if the need ever arises.

---

## Usage

### Building

```bash
cargo build --release
```

The binary will be located in `target/release/fixFileExt`.

### Executing Project

This only applies when wanting to run the project on the source directly without building it first.

```bash
cargo run <directory>
```

### Executing Standalone

This only applies when wanting to run the standalone executable produced in the `target/release` directory once built.

```bash
./fixFileExt <directory>
```
