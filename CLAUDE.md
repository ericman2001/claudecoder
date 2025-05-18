# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Status

This is a Rust command line application that computes the arithmetic mean of numbers in a file.

## Application Goal

The application accepts a file path as a parameter and computes the arithmetic mean of the numbers in the file.

## Project Structure

- `src/main.rs` - Main application code (well-commented)
- `src/tests.rs` - Unit tests (comprehensive test documentation)
- `Cargo.toml` - Project configuration and dependencies

## Build and Run Commands

1. Build the project:
   ```
   cargo build
   ```

2. Run the program:
   ```
   cargo run -- <file_path>
   ```

3. Run tests:
   ```
   cargo test
   ```

## Requirements Met

1. ✓ Prints a message if no file path is provided
2. ✓ Avoids divide by zero error when computing the mean
3. ✓ Outputs the arithmetic mean of values in the file
4. ✓ Assumes one number per line
5. ✓ Ignores blank lines and non-numeric values without error
6. ✓ Well-architected for unit tests
7. ✓ Unit tests for all required scenarios:
   - No file path supplied
   - Blank file supplied
   - Valid file with numeric values

## Dependencies

- `tempfile` (dev-dependency) - Used for creating temporary files in tests