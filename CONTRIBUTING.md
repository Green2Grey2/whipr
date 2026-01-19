# Contributing to Whipr

Thank you for your interest in contributing to Whipr! This document provides guidelines and information for contributors.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/whipr.git`
3. Install dependencies (see [README.md](README.md#development))
4. Create a branch: `git checkout -b feature/your-feature`

## Development Setup

### Prerequisites

- Rust (stable)
- Node.js 18+
- Platform-specific dependencies (see README)

### Running Locally

```bash
npm install
npm run tauri dev
```

## Code Style

### Rust

- Follow standard Rust conventions
- Use `cargo fmt` before committing
- Run `cargo clippy` to catch common issues

### TypeScript/Svelte

- Use TypeScript for type safety
- Follow existing component patterns
- Keep components focused and reusable

## Commit Messages

Use clear, descriptive commit messages:

```
feat: add voice activity detection
fix: resolve audio buffer overflow on long recordings
docs: update installation instructions for Fedora
refactor: simplify hotkey registration logic
```

Prefixes:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

## Pull Requests

1. Update documentation if needed
2. Test on your platform
3. Ensure `cargo build` and `npm run build` succeed
4. Write a clear PR description explaining your changes

### PR Checklist

- [ ] Code follows project style guidelines
- [ ] Changes are tested locally
- [ ] Documentation is updated (if applicable)
- [ ] Commit messages are clear and descriptive

## Reporting Issues

When reporting bugs, please include:

- Operating system and version
- Desktop environment (GNOME, KDE, etc.)
- Display server (X11 or Wayland)
- Steps to reproduce
- Expected vs actual behavior
- Relevant logs or error messages

## Feature Requests

Feature requests are welcome! Please:

- Check existing issues first
- Describe the use case
- Explain why it would benefit users

## Areas for Contribution

### Good First Issues

Look for issues labeled `good first issue` for beginner-friendly tasks.

### Priority Areas

- Platform testing (especially Wayland, Windows, macOS)
- Documentation improvements
- Accessibility enhancements
- Performance optimizations
- Translation/localization

## Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md) for technical details about the codebase.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
