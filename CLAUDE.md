# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Status

This is a Rust command line application that computes either the arithmetic mean or sum of numbers in a file.

## Application Goal

The application accepts an operation type (mean or sum) and a file path as parameters, then computes the specified calculation on the numbers in the file.

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
   cargo run -- <operation> <file_path>
   ```
   Where `<operation>` is either `mean` or `sum`

3. Run tests:
   ```
   cargo test
   ```

## Requirements Met

1. ✓ Prints usage message if insufficient parameters are provided
2. ✓ Supports both arithmetic mean and sum operations
3. ✓ Avoids divide by zero error when computing the mean
4. ✓ Outputs the calculated result (mean or sum) of values in the file
5. ✓ Assumes one number per line
6. ✓ Ignores blank lines and non-numeric values without error
7. ✓ Well-architected for unit tests
8. ✓ Unit tests for all required scenarios:
   - Insufficient arguments supplied
   - Blank file supplied (both mean and sum)
   - Valid file with numeric values (both mean and sum)
   - Files with mixed content (both mean and sum)

## Dependencies

- `tempfile` (dev-dependency) - Used for creating temporary files in tests