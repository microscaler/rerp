# Contributing to RERP

Thank you for your interest in contributing to **RERP** (Rust Enterprise Resource Planning)! 🎉

This guide will help you get started with contributing to the project.

---

## Getting Started

### Prerequisites

- **Rust toolchain** (stable) - [Install Rust](https://www.rust-lang.org/tools/install)
- **Python 3.12+** (for generation scripts)
- **Git** for version control
- **BRRTRouter** - [BRRTRouter repository](https://github.com/microscaler/BRRTRouter) (for code generation)

### Quick Start

1. **Fork and clone the repository**:
   ```bash
   git clone https://github.com/microscaler/rerp.git
   cd rerp
   ```

2. **Explore the project structure**:
   ```bash
   # View service specifications
   ls openapi/*/openapi.yaml
   
   # Check crate structure
   ls components/*/
   ```

3. **Generate system BFF specs** (optional, for testing): run `just init` then `rerp bff generate-system`.

---

## Development Workflow

### 1. Create a Feature Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Your Changes

- Follow Rust best practices and conventions
- Ensure code compiles without warnings
- Add tests for new functionality
- Update documentation as needed

### 3. Test Your Changes

```bash
# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Format code
cargo fmt

# Check linting
cargo clippy --workspace
```

### 4. Commit Your Changes

Commit messages must follow the [Conventional Commits](https://www.conventionalcommits.org/) specification. This is enforced in CI via commitlint; PRs with invalid commit messages will fail.

Format: `<type>[optional scope]: <description>`

Examples:
- `feat: add new service endpoint`
- `feat(auth): add login endpoint`
- `fix: correct validation logic`
- `docs: update API documentation`
- `test: add tests for inventory service`
- `chore(deps): bump crate-x to 1.2`

Common types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `chore`, `ci`, `build`, `revert`.

### 5. Submit a Pull Request

- Push your branch to your fork
- Create a pull request with a clear description
- Reference any related issues
- Ensure CI checks pass

---

## Areas for Contribution

We welcome contributions in the following areas:

### OpenAPI Specifications

- **Enhance service specs**: Add complete schemas, examples, and detailed descriptions
- **Add missing endpoints**: Identify and document additional API endpoints
- **Improve documentation**: Enhance service descriptions and parameter documentation

### Service Implementation

- **Implement business logic**: Add handlers and controllers for services
- **Add validation**: Implement request/response validation
- **Error handling**: Improve error handling and user feedback

### Testing

- **Unit tests**: Add tests for individual services and functions
- **Integration tests**: Test service interactions and workflows
- **Load testing**: Performance and scalability testing

### Documentation

- **User guides**: Create guides for using RERP services
- **API documentation**: Improve OpenAPI specifications
- **Code examples**: Add practical usage examples

### CI/CD

- **Enhance automation**: Improve GitHub Actions workflows
- **Add checks**: Additional quality checks and validations
- **Deployment**: Improve deployment processes

---

## Code Standards

### Rust Code

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Run `cargo clippy` and fix warnings
- Write comprehensive doc comments
- Maintain test coverage (target: 65% minimum, 80% preferred)

### OpenAPI Specifications

- Use OpenAPI 3.1.0 format
- Include complete path definitions
- Define request/response schemas
- Add operation summaries and descriptions
- Follow RESTful conventions

### Git Workflow

- Use [Conventional Commits](https://www.conventionalcommits.org/) (enforced in CI)
- Keep commits focused and atomic
- Rebase on main before submitting PRs
- Write clear PR descriptions

---

## Project Structure

```
rerp/
├── components/          # Rust workspace with service crates
│   ├── {system}/       # System directories
│   │   ├── {module}/   # Generated crate
│   │   └── {module}_impl/  # Implementation crate
│   └── common/         # Shared utilities
├── openapi/            # OpenAPI specifications
│   ├── {system}/       # System directories
│   │   ├── openapi.yaml  # System BFF spec
│   │   └── {module}/   # Service directories
├── tooling/            # rerp CLI (ports, openapi, ci, bff, build, docker, bootstrap, tilt)
├── docs/               # Documentation
└── .github/            # GitHub workflows
```

See [components/README.md](components/README.md) for detailed crate structure information.

---

## Testing Requirements

### Before Submitting

- ✅ All tests pass: `cargo test --workspace`
- ✅ Code compiles: `cargo build --workspace`
- ✅ No warnings: `cargo clippy --workspace`
- ✅ Formatted: `cargo fmt --check`
- ✅ Documentation builds: `cargo doc --workspace`

### Test Coverage

- Minimum coverage: **65%**
- Target coverage: **80%**
- Critical paths: **100%**

---

## Questions?

- **Issues**: [GitHub Issues](https://github.com/microscaler/rerp/issues)
- **Discussions**: [GitHub Discussions](https://github.com/microscaler/rerp/discussions)

---

## Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please be respectful and constructive in all interactions.

---

Thank you for contributing to RERP! 🚀
