# Contributing to voicepeak-cli

Thank you for your interest in contributing to voicepeak-cli! We welcome contributions from everyone.

## Bug Reports and Feature Requests

- Use the [GitHub Issues](https://github.com/petamorikei/voicepeak-cli/issues) to report bugs or request features
- Please search existing issues before creating a new one
- Provide as much detail as possible, including:
  - Your operating system and version
  - VOICEPEAK version
  - Steps to reproduce the issue
  - Expected vs actual behavior

## Code Contributions

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature-name`
3. Make your changes
4. Add tests if applicable
5. Run the test suite: `cargo test`
6. Check formatting: `cargo fmt`
7. Run clippy: `cargo clippy`
8. Commit your changes: `git commit -am 'Add some feature'`
9. Push to the branch: `git push origin feature/your-feature-name`
10. Create a Pull Request

## Development Setup

```bash
# Clone the repository
git clone https://github.com/petamorikei/voicepeak-cli.git
cd voicepeak-cli

# Build the project
cargo build

# Run tests
cargo test

# Install locally for testing
cargo install --path .
```

## Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Ensure all clippy lints pass (`cargo clippy`)
- Write tests for new functionality
- Update documentation as needed

## Commit Message Guidelines

- Use clear and descriptive commit messages
- Start with a verb in imperative mood (e.g., "Add", "Fix", "Update")
- Keep the first line under 50 characters
- Reference issues when applicable (e.g., "Fix #123")

## Pull Request Process

1. Update the README.md with details of changes if applicable
2. Update the version numbers in Cargo.toml following [Semantic Versioning](https://semver.org/)
3. The PR will be merged once you have the sign-off of a maintainer

## Questions?

If you have any questions about contributing, feel free to open an issue or reach out to the maintainers.

Thank you for contributing!