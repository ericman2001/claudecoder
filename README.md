# Claudecoder - A Claude Code Exploration Repository

This repository serves as a sandbox and exploration tool for **Claude Code** ([claude.ai/code](https://claude.ai/code)), Anthropic's AI-powered coding assistant. We're using this project to explore Claude Code's capabilities through a simple yet practical Rust application.

## Project Purpose

This is a Rust command-line application that calculates either the arithmetic mean or sum of numbers in a file. While the application itself is straightforward, it demonstrates various software engineering best practices and serves as a testbed for Claude Code's features.

## What This Repository Demonstrates

- **AI-Assisted Development**: Created entirely through conversation with Claude Code
- **Test-Driven Development**: Comprehensive unit tests for all functionality
- **Code Organization**: Well-structured Rust code with separated concerns
- **Documentation**: Thoroughly commented code for maintainability
- **Error Handling**: Robust handling of edge cases and errors

## Application Features

The numeric calculator:
- Accepts an operation type (mean or sum) and file path as command-line arguments
- Reads numeric values from the file (one per line)
- Ignores blank lines and non-numeric content
- Calculates and displays either the arithmetic mean or sum
- Handles edge cases (empty files, missing arguments) gracefully

## Usage

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Building

```bash
cargo build
```

### Running

```bash
cargo run -- <operation> <path_to_file>
```

Where `<operation>` is either `mean` or `sum`.

Examples:
```bash
echo -e "10\n20\n30" > numbers.txt
cargo run -- mean numbers.txt
# Output: 20

cargo run -- sum numbers.txt
# Output: 60
```

### Testing

```bash
cargo test
```

## Project Structure

```
claudecoder/
├── src/
│   ├── main.rs   # Main application logic
│   └── tests.rs  # Unit tests
├── Cargo.toml    # Project configuration
├── CLAUDE.md     # Claude Code specific documentation
└── README.md     # This file
```

## Claude Code Development Process

This repository showcases how Claude Code can:
1. Initialize a project from scratch
2. Implement functionality based on requirements
3. Write comprehensive tests
4. Add detailed documentation
5. Refactor code for better organization
6. Respond to feedback and make improvements

## Contributing

As this is primarily a demonstration repository for Claude Code, contributions should focus on:
- Exploring new Claude Code capabilities
- Documenting interesting AI-assisted development patterns
- Testing edge cases in AI code generation

## Learning Resources

- [Claude Code Documentation](https://docs.anthropic.com/en/docs/claude-code)
- [Rust Programming Language](https://www.rust-lang.org/)
- [Anthropic Claude](https://www.anthropic.com/claude)

## License

This project is created for educational and demonstration purposes. Feel free to use the code as a reference for your own Claude Code explorations.

---

*This repository was created as an exploration of Claude Code's capabilities. All code was generated through AI-assisted development with human guidance.*