# Rust By Example

This repository features activities from the [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
book.

All activity files are in `./src` and are organized as follows:

- file names correspond to the course name (e.g. `primites.rs`, `custom_types.rs`, etc.)
- assertions to test the correctness of the activities are implemented using [rstest](https://docs.rs/rstest/latest/rstest/)
and are included in the same file with the actual code. This seems to be Rust's suggested
approach.
