# 🤝 Contributing to QR Code Generator API

Thank you for your interest in contributing to the QR Code Generator API! This guide will help you get started with contributing to the project.

## 📋 Table of Contents

- [Code of Conduct](#-code-of-conduct)
- [Getting Started](#-getting-started)
- [Development Process](#-development-process)
- [Pull Request Process](#-pull-request-process)
- [Coding Standards](#-coding-standards)
- [Testing Guidelines](#-testing-guidelines)
- [Documentation](#-documentation)
- [Issue Guidelines](#-issue-guidelines)
- [Community](#-community)

## 📜 Code of Conduct

### Our Pledge

We are committed to providing a friendly, safe, and welcoming environment for all contributors, regardless of experience level, gender identity, sexual orientation, disability, personal appearance, body size, race, ethnicity, age, religion, or nationality.

### Expected Behavior

- Be respectful and inclusive
- Welcome newcomers and help them get started
- Focus on constructive criticism
- Accept feedback gracefully
- Be mindful of your language

## 🚀 Getting Started

### Prerequisites

- Rust 1.70 or higher
- Git
- A GitHub account
- Basic understanding of REST APIs
- Familiarity with Rust is helpful but not required

### Setting Up Your Development Environment

1. **Fork the Repository**
   ```bash
   # Navigate to https://github.com/Tolstoyj/QRCodeGeneratorApi
   # Click the "Fork" button
   ```

2. **Clone Your Fork**
   ```bash
   git clone https://github.com/YOUR_USERNAME/QRCodeGeneratorApi.git
   cd qr-api
   ```

3. **Add Upstream Remote**
   ```bash
   git remote add upstream https://github.com/Tolstoyj/QRCodeGeneratorApi.git
   ```

4. **Install Dependencies**
   ```bash
   cargo build
   ```

5. **Run Tests**
   ```bash
   cargo test
   ```

6. **Start Development Server**
   ```bash
   cargo run
   ```

## 💻 Development Process

### Branch Naming Convention

- `feature/` - New features (e.g., `feature/add-webp-support`)
- `fix/` - Bug fixes (e.g., `fix/color-validation-error`)
- `docs/` - Documentation updates (e.g., `docs/update-api-examples`)
- `refactor/` - Code refactoring (e.g., `refactor/optimize-qr-generation`)
- `test/` - Test additions/updates (e.g., `test/add-integration-tests`)

### Development Workflow

1. **Create a New Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make Your Changes**
   - Write clean, documented code
   - Follow the existing code style
   - Add tests for new functionality

3. **Test Your Changes**
   ```bash
   # Run all tests
   cargo test
   
   # Run specific test
   cargo test test_name
   
   # Run with coverage
   cargo tarpaulin --out Html
   ```

4. **Format Your Code**
   ```bash
   cargo fmt
   ```

5. **Lint Your Code**
   ```bash
   cargo clippy -- -D warnings
   ```

6. **Commit Your Changes**
   ```bash
   git add .
   git commit -m "feat: add WebP format support
   
   - Implemented WebP encoding
   - Added tests for WebP generation
   - Updated documentation"
   ```

### Commit Message Guidelines

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Test additions or changes
- `chore`: Maintenance tasks
- `perf`: Performance improvements

**Examples:**
```bash
feat(v2): add SVG format support
fix(validation): correct color contrast calculation
docs(api): update v2 endpoint examples
refactor(handlers): simplify error handling logic
```

## 🔄 Pull Request Process

### Before Submitting

1. **Update from Upstream**
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Run Full Test Suite**
   ```bash
   cargo test
   cargo fmt --check
   cargo clippy
   ```

3. **Update Documentation**
   - Update API_DOCS.md if you've changed endpoints
   - Update README.md if needed
   - Add inline documentation for new functions

### Submitting a Pull Request

1. **Push to Your Fork**
   ```bash
   git push origin feature/your-feature-name
   ```

2. **Create Pull Request**
   - Go to the original repository
   - Click "New Pull Request"
   - Select your branch
   - Fill out the PR template

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] All tests pass
- [ ] Added new tests
- [ ] Manual testing completed

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-reviewed code
- [ ] Updated documentation
- [ ] No new warnings
```

### Review Process

1. **Automated Checks**: CI/CD will run tests automatically
2. **Code Review**: Maintainers will review your code
3. **Feedback**: Address any requested changes
4. **Merge**: Once approved, your PR will be merged

## 📏 Coding Standards

### Rust Style Guide

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Prefer explicit over implicit
- Write self-documenting code

### Code Organization

```rust
// Good: Clear module organization
mod config;
mod handlers;
mod models;
mod services;

// Good: Clear function signature
pub async fn generate_qr(
    url: String,
    options: QrOptions,
) -> Result<QrCode, QrError> {
    // Implementation
}
```

### Documentation

```rust
/// Generates a QR code from the provided URL
///
/// # Arguments
///
/// * `url` - The URL to encode in the QR code
/// * `options` - Customization options for the QR code
///
/// # Returns
///
/// Returns a `Result` containing the generated QR code or an error
///
/// # Examples
///
/// ```
/// let qr = generate_qr("https://example.com", Default::default())?;
/// ```
pub fn generate_qr(url: &str, options: QrOptions) -> Result<QrCode, QrError> {
    // Implementation
}
```

### Error Handling

```rust
// Good: Descriptive error types
#[derive(Debug, thiserror::Error)]
pub enum QrError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    
    #[error("Generation failed: {0}")]
    GenerationFailed(String),
}

// Good: Proper error propagation
fn process_url(url: &str) -> Result<String, QrError> {
    validate_url(url)?;
    sanitize_url(url)
}
```

## 🧪 Testing Guidelines

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_url() {
        // Arrange
        let url = "https://example.com";
        
        // Act
        let result = validate_url(url);
        
        // Assert
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_async_operation() {
        // Async test implementation
    }
}
```

### Test Coverage

- Aim for >80% code coverage
- Test edge cases and error conditions
- Include integration tests for API endpoints
- Add property-based tests where applicable

### Running Tests

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run specific test module
cargo test handlers::

# Run with coverage
cargo tarpaulin --out Html
```

## 📚 Documentation

### When to Update Documentation

- Adding new features
- Changing API behavior
- Modifying configuration options
- Fixing documentation errors

### Documentation Standards

1. **API Documentation**: Update API_DOCS.md
2. **Code Comments**: Add inline documentation
3. **README Updates**: Keep examples current
4. **ROADMAP Updates**: Update feature status

### Documentation Checklist

- [ ] API endpoints documented
- [ ] Request/response examples provided
- [ ] Error codes documented
- [ ] Configuration options explained
- [ ] Migration guide updated (if breaking changes)

## 🐛 Issue Guidelines

### Reporting Bugs

**Title**: Clear, concise description  
**Example**: "Color validation fails for 3-digit hex codes"

**Body**:
```markdown
## Description
Brief description of the bug

## Steps to Reproduce
1. Send request to /v2/generate
2. Use color "#F00"
3. Observe error

## Expected Behavior
Should accept 3-digit hex codes

## Actual Behavior
Returns validation error

## Environment
- OS: macOS 12.0
- Rust: 1.70
- API Version: 2.0.0
```

### Feature Requests

**Title**: "Feature: [Brief description]"  
**Example**: "Feature: Add WebP format support"

**Body**:
```markdown
## Problem
Current limitation or use case

## Proposed Solution
How the feature would work

## Alternatives Considered
Other approaches explored

## Additional Context
Any relevant information
```

## 🌟 Recognition

### Contributors

All contributors will be recognized in:
- README.md contributors section
- GitHub contributors page
- Release notes

### Types of Contributions

We value all contributions:
- 💻 Code contributions
- 📖 Documentation improvements
- 🐛 Bug reports
- 💡 Feature suggestions
- 🧪 Test additions
- 🎨 Design improvements
- 👥 Community support

## 🤔 Getting Help

### Resources

- [API Documentation](API_DOCS.md)
- [Development Setup](README.md#-installation)
- [Architecture Overview](ARCHITECTURE.md)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Axum Documentation](https://docs.rs/axum/)

### Community

- **GitHub Issues**: For bugs and features
- **GitHub Discussions**: For questions and ideas
- **Email**: devops@deepplaystudio.com

## 📋 Checklist for Contributors

Before submitting your PR, ensure:

- [ ] Code compiles without warnings
- [ ] All tests pass
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation updated
- [ ] Commit messages follow convention
- [ ] PR description is complete
- [ ] Branch is up-to-date with main

## 🎉 Thank You!

Thank you for contributing to the QR Code Generator API! Your efforts help make this project better for everyone.

---

**Questions?** Feel free to open an issue or reach out to the maintainers.

**Happy Coding!** 🚀