# <FILE>Justfile</FILE> - <DESC>Project task runner commands</DESC>
# <VERS>VERSION: 0.1.0</VERS>
# <WCTX>Add task runner for common development workflows</WCTX>
# <CLOG>Initial creation with demo, build, test, and check commands</CLOG>

# Default recipe - show available commands
default:
    @just --list

# Run the recipe demo browser in release mode
demo:
    cargo run --example demo --release

# Run the recipe demo in debug mode (faster compile, slower runtime)
demo-debug:
    cargo run --example demo

# Build the project
build:
    cargo build

# Build in release mode
build-release:
    cargo build --release

# Run all tests
test:
    cargo test

# Run clippy lints
clippy:
    cargo clippy --all-targets

# Check formatting
fmt-check:
    cargo fmt -- --check

# Format code
fmt:
    cargo fmt

# Full check (clippy + fmt + test)
check: clippy fmt-check test

# Clean build artifacts
clean:
    cargo clean

# <FILE>Justfile</FILE> - <DESC>Project task runner commands</DESC>
# <VERS>END OF VERSION: 0.1.0</VERS>
