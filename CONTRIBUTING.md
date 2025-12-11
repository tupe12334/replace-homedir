# Contributing to replace-homedir

Thank you for your interest in contributing to this project!

## Project Goal

This library aims to be an **exact Rust replica** of the npm [replace-homedir](https://www.npmjs.com/package/replace-homedir) package. When contributing, please ensure that any changes maintain API compatibility and behavioral parity with the original npm package.

## Development Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/tupe12334/replace-homedir.git
   cd replace-homedir
   ```

2. Ensure you have the Rust toolchain installed:
   ```bash
   rustup update stable
   ```

3. Install required components:
   ```bash
   rustup component add rustfmt clippy
   ```

## Development Workflow

### Running Tests

```bash
cargo test --all-features
cargo test --doc
```

### Linting

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

### Building

```bash
cargo build --release
```

## Submitting Changes

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Make your changes
4. Ensure all tests pass and linting is clean
5. Commit your changes with a descriptive message
6. Push to your fork and open a Pull Request against `main`

## Publishing a New Version

The CI/CD pipeline automatically handles publishing to crates.io. To release a new version:

1. Update the version in `Cargo.toml`
2. Commit the version bump: `git commit -am "chore: bump version to X.Y.Z"`
3. Push to `main` (or merge a PR into `main`)

The CI workflow will:
- Run lint checks (formatting + Clippy)
- Run all tests
- Build the release binary
- Check if the version already exists on crates.io
- Publish to crates.io if the version is new

Note: Publishing requires the `CARGO_REGISTRY_TOKEN` secret to be configured in the repository.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
