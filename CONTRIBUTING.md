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
   ls microservices/*/
   ```

3. **Generate system BFF specs** (optional, for testing): run `just init` then `rerp bff generate-system`.

4. **Pre-commit hooks** (recommended): run **`just pre-commit-setup`** once (init + npm install + install-hooks). Or manually: `just init` then `just install-hooks`, and `npm install` at repo root for commit message lint. Before each commit this runs:
   - **Tooling QA**: `just qa` (lint, format-check, tooling tests)
   - **microservices-fmt**: if `microservices/` changed vs HEAD, runs `just fmt-rust` (cargo fmt in `microservices/`, rustfmt in `entities/`); fast if Tilt has recently built.
   - **commitlint** (commit-msg): validates conventional commit format using the same config as CI (`commitlint.config.mjs`).

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

Format: `<type>[optional scope]: <description>` (subject line max 1500 characters; extra space for agentic tracking)

Examples:
- `feat: add new service endpoint`
- `feat(auth): add login endpoint`
- `fix: correct validation logic`
- `docs: update API documentation`
- `test: add tests for inventory service`
- `chore(deps): bump crate-x to 1.2`

Common types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `chore`, `ci`, `build`, `revert`, `tooling`.

The pre-commit hook runs commitlint automatically on each commit (if you ran `just install-hooks` and `npm install`). To validate the last commit manually: `npx commitlint --from HEAD~1`.

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
- **Planning documents**: Design proposals, PRDs, and analysis documents (see "Planning Documents" section below)

### CI/CD

- **Enhance automation**: Improve GitHub Actions workflows
- **Add checks**: Additional quality checks and validations
- **Deployment**: Improve deployment processes

---

## Planning Documents

**⚠️ CRITICAL: All planning, analysis, design proposals, and implementation status documents MUST be created in `./docs/` or its subdirectories.**

### Document Organization

- **`docs/ai/`** - AI-generated planning, analysis, and implementation status documents
- **`docs/adrs/`** - Architecture Decision Records (ADRs)
- **`docs/`** (root) - Design proposals, PRDs, and other planning documents

### Naming Conventions

- Design proposals: `docs/DESIGN_PROPOSAL_*.md`
- PRDs: `docs/*_PRD.md`
- Analysis documents: `docs/*_ANALYSIS.md`
- Status documents: `docs/*_STATUS.md` or `docs/*_COMPLETE.md`
- Implementation status: `docs/ai/*_COMPLETE.md` or `docs/ai/*_STATUS.md`

### Examples

✅ **Correct locations:**
- `docs/DESIGN_PROPOSAL_RELEASE_CI_INTEGRATION.md`
- `docs/VERSIONING_STRATEGY_ANALYSIS.md`
- `docs/ACCOUNTING_SUITE_ENRICHMENT_PRD.md`
- `docs/ai/OPENAPI_GENERATION_COMPLETE.md`

❌ **Incorrect locations (NOT ALLOWED):**
- `DESIGN_PROPOSAL_*.md` (project root)
- `VERSIONING_STRATEGY_*.md` (project root)
- `ANALYSIS_*.md` (project root)

### Why This Matters

- Keeps project root clean and organized
- Makes documentation easy to find
- Follows standard documentation structure
- Prevents accidental commits of planning documents in wrong locations

**If you find planning documents in the project root, move them to `./docs/` immediately.**

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
├── microservices/      # Rust workspace with service crates
│   ├── {suite}/        # Suite directories (e.g., accounting, hr, sales)
│   │   ├── {service}/  # Service directories
│   │   │   ├── gen/    # Generated crate (from OpenAPI)
│   │   │   └── impl/   # Implementation crate (business logic)
├── openapi/            # OpenAPI specifications
│   ├── {suite}/        # Suite directories
│   │   ├── bff-suite-config.yaml  # Suite BFF config
│   │   ├── openapi_bff.yaml       # Generated suite BFF spec
│   │   └── {service}/  # Service directories
│   │       └── openapi.yaml
├── tooling/            # rerp CLI (ports, openapi, ci, bff, build, docker, bootstrap, tilt)
├── docs/               # Documentation (ALL planning documents go here)
│   ├── ai/             # AI-generated planning and analysis
│   ├── adrs/           # Architecture Decision Records
│   └── *.md            # Design proposals, PRDs, analysis documents
└── .github/            # GitHub workflows
```

**⚠️ Important**: The project root should NOT contain planning documents. All planning, analysis, design proposals, and status documents must be in `./docs/` or subdirectories.

See microservices structure: `microservices/{suite}/{service}/gen/` (generated) and `microservices/{suite}/{service}/impl/` (business logic).

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
