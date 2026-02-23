# Verbose FizzBuzz - A Comprehensive Rust Implementation

This is a verbose, well-documented FizzBuzz application written in Rust. The project demonstrates Rust fundamentals with extensive inline documentation and comments.

## What is FizzBuzz?

FizzBuzz is a classic programming challenge that tests:
- Loop iteration
- Conditional logic
- String manipulation
- Divisibility testing

### Rules
- Print numbers from 1 to 100
- For multiples of 3, print "Fizz" instead of the number
- For multiples of 5, print "Buzz" instead of the number
- For multiples of both 3 and 5, print "FizzBuzz" instead of the number

## Project Structure

```
verbose-system/
├── Cargo.toml           # Cargo configuration and project metadata
├── Cargo.lock          # Dependency lock file (if applicable)
├── src/
│   └── main.rs         # Main FizzBuzz implementation
├── .gitignore          # Git ignore rules
└── README.md           # This file
```

## Prerequisites

Before building and running this project, ensure you have Rust and Cargo installed:

```bash
# Install Rust using rustup (recommended)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

For more information, visit https://www.rust-lang.org/tools/install

## Building the Project

### Development Build

To build the project in debug mode (faster compilation, slower runtime):

```bash
cargo build
```

The compiled binary will be located at `target/debug/verbose-system`

### Release Build

To build an optimized release binary (slower compilation, faster runtime):

```bash
cargo build --release
```

The compiled binary will be located at `target/release/verbose-system`

## Running the Application

### Using Cargo (Recommended)

To compile and run the application directly:

```bash
cargo run
```

For release mode:

```bash
cargo run --release
```

### Using the Compiled Binary

After building, you can run the compiled binary directly:

**Debug binary:**
```bash
./target/debug/verbose-system
```

**Release binary:**
```bash
./target/release/verbose-system
```

## Running Tests

To execute all unit tests:

```bash
cargo test
```

To run tests with verbose output:

```bash
cargo test -- --nocapture
```

To run a specific test:

```bash
cargo test test_fizzbuzz_divisible_by_both
```

## Project Features

✅ **Verbose Documentation** - Extensive comments explaining the code  
✅ **Multiple Implementations** - Both if-else and pattern matching approaches  
✅ **Comprehensive Tests** - Unit tests for all FizzBuzz scenarios  
✅ **Proper Cargo Structure** - Follows Rust project conventions  
✅ **Clean Build System** - Complete boilerplate for building and testing  

## Code Overview

### Main Components

1. **`main()`** - Application entry point
2. **`fizzbuzz()`** - Core FizzBuzz iteration logic
3. **`determine_fizzbuzz_output()`** - FizzBuzz rule logic (if-else version)
4. **`determine_fizzbuzz_output_pattern_match()`** - Alternative pattern matching version
5. **`tests` module** - Unit tests for all scenarios

## Additional Cargo Commands

```bash
# Clean build artifacts
cargo clean

# Check code without building
cargo check

# Generate documentation
cargo doc --open

# Format code (requires rustfmt)
cargo fmt

# Lint code (requires clippy)
cargo clippy

# Build and run in one command
cargo run

# Run with specific features
cargo build --features "feature-name"
```

## Performance Notes

The release build configuration in `Cargo.toml` includes optimizations:
- `opt-level = 3` - Maximum optimization
- `lto = true` - Link-time optimization
- `codegen-units = 1` - Better optimization at the cost of longer compilation time

## Troubleshooting

### Build Issues

If you encounter build errors, try:

```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build
```

### Permission Denied on Binary

On Unix-like systems, make the binary executable:

```bash
chmod +x target/debug/verbose-system
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## References

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)