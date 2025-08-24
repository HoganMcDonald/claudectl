# Contributing to ClaudeCtl

## Development Setup

1. **Install Rust**: [rustup.rs](https://rustup.rs/)
2. **Clone the repository**:
   ```bash
   git clone https://github.com/your-org/claudectl.git
   cd claudectl
   ```
3. **Build the project**:
   ```bash
   cargo build
   ```
4. **Run tests**:
   ```bash
   cargo test
   ```

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix all warnings
- Follow existing code patterns and naming conventions
- Add documentation for public APIs

## Testing

- Write tests for new functionality
- Ensure all existing tests pass
- Integration tests are in `tests/` directory
- Unit tests should be in the same file as the code they test

## Pull Request Process

1. Create a feature branch from `main`
2. Make your changes
3. Add or update tests as needed
4. Run the full test suite: `cargo test`
5. Run linting: `cargo clippy -- -D warnings`
6. Format code: `cargo fmt`
7. Submit a pull request

## Issues

- Use GitHub Issues for bugs and feature requests
- Provide clear reproduction steps for bugs
- Check existing issues before creating new ones