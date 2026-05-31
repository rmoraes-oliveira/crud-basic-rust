# Contributing to Notes API

Thank you for your interest in contributing to the Notes API project! This document provides guidelines and instructions for contributing.

## Getting Started

### Prerequisites
- Rust 1.70+ installed
- PostgreSQL 12+ or Docker
- Git

### Setup Development Environment

1. Clone the repository:
```bash
git clone <repository-url>
cd notes-api
```

2. Create a `.env` file from the template:
```bash
cp .env.example .env
```

3. Setup the database:
```bash
make dev-setup
```

4. Run the development server:
```bash
make run
```

## Development Workflow

### Creating a Feature Branch

Always create a new branch for your work:
```bash
git checkout -b feature/your-feature-name
```

Use clear branch names:
- `feature/add-search-notes` - New features
- `fix/connection-timeout` - Bug fixes
- `docs/update-readme` - Documentation
- `refactor/error-handling` - Code improvements

### Code Style

#### Formatting
```bash
cargo fmt
```

All code must be formatted before committing.

#### Linting
```bash
cargo clippy
```

Fix all clippy warnings. Use `#[allow(...)]` only when justified.

#### Testing
```bash
cargo test
```

Write tests for new functionality. Aim for at least 80% code coverage.

### Commit Messages

Write clear, descriptive commit messages:
```
[Type] Brief description

Detailed explanation of what changed and why.

Fixes #123
```

Types:
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation
- `style:` - Code formatting
- `refactor:` - Code refactoring
- `test:` - Test additions/changes
- `chore:` - Build, dependencies, etc.

Example:
```
feat: Add note filtering by date range

- Add filter parameters to GET /notes endpoint
- Implement date parsing utility
- Add database query optimization
- Add tests for date filtering

Closes #42
```

## Adding New Features

### 1. Create the Feature Branch
```bash
git checkout -b feature/new-feature
```

### 2. Add Database Support (if needed)
```bash
sqlx migrate add -r migration_name
```

Edit the migration file in `migrations/` directory.

### 3. Update Models
Add new fields to `src/models.rs` if needed.

### 4. Implement Database Layer
Add functions to `src/db/notes.rs`.

### 5. Add HTTP Handlers
Add handlers to `src/routes/notes.rs`.

### 6. Update Routes
Modify `src/routes/mod.rs` to register new endpoints.

### 7. Write Tests
```bash
cargo test
```

### 8. Update Documentation
- Update API endpoints in README.md
- Add examples if applicable
- Update PROJECT_STRUCTURE.md if needed

### 9. Run Quality Checks
```bash
cargo fmt
cargo clippy
cargo test
cargo check
```

### 10. Commit and Push
```bash
git add .
git commit -m "feat: your feature description"
git push origin feature/new-feature
```

## Reporting Bugs

When reporting bugs, include:

1. **Description**: Clear explanation of the bug
2. **Steps to Reproduce**: Exact steps to trigger the bug
3. **Expected Behavior**: What should happen
4. **Actual Behavior**: What actually happens
5. **Environment**: 
   - OS and version
   - Rust version (`rustx --version`)
   - PostgreSQL version
6. **Error Output**: Full error message if applicable
7. **Logs**: Relevant application logs

Example:
```
**Title:** API returns 500 error when creating note with long content

**Description:** When creating a note with content longer than 10000 characters, the server returns a 500 error instead of validating the input.

**Steps to Reproduce:**
1. Start the API server
2. Send POST request to /notes with content of 10001 characters
3. API returns 500 error

**Expected:** API should validate input and return 400 with descriptive error

**Environment:** 
- macOS 14.0
- Rust 1.70.0
- PostgreSQL 15

**Error:**
```
thread 'tokio-runtime-worker' panicked at 'error: ...'
```
```

## Pull Request Process

### Before Submitting

1. Update your branch with latest changes:
```bash
git fetch origin
git rebase origin/main
```

2. Run all quality checks:
```bash
make lint test fmt
```

3. Update documentation if needed

4. Create a descriptive PR title and description

### PR Description Template

```markdown
## Description
Brief description of the changes

## Type of Change
- [ ] New feature
- [ ] Bug fix
- [ ] Documentation update
- [ ] Code refactoring

## Related Issues
Fixes #(issue number)

## Testing
Describe testing performed

## Checklist
- [ ] Code follows style guidelines (`cargo fmt`)
- [ ] Code passes linting (`cargo clippy`)
- [ ] New tests added/updated
- [ ] Documentation updated
- [ ] Database migration added (if applicable)
- [ ] No breaking changes
```

### PR Review Process

PRs will be reviewed for:
- ✅ Code quality and style
- ✅ Test coverage
- ✅ Documentation completeness
- ✅ Breaking changes
- ✅ Security implications
- ✅ Performance impact

Feedback will be constructive. Be prepared to iterate on your PR.

## Code Review Guidelines

When reviewing code:
- Be respectful and constructive
- Ask questions to understand the approach
- Suggest improvements politely
- Approve when satisfied
- Use the "Request Changes" status if major issues found

## Project Structure Guidelines

### Adding New Modules

When adding new functionality:

1. Create appropriate module file in `src/`
2. Export from parent `mod.rs`
3. Add unit tests in the same file
4. Document public APIs with doc comments

Example:
```rust
/// List all notes from the database.
///
/// # Arguments
/// * `db` - Database connection pool
///
/// # Returns
/// * `Ok(Vec<Note>)` - List of notes
/// * `Err(AppError)` - Database error
pub async fn list(db: &PgPool) -> Result<Vec<Note>, AppError> {
    // implementation
}
```

### Database Migrations

- Create reversible migrations: `sqlx migrate add -r name`
- Use descriptive names
- Include rollback logic
- Test both up and down migrations

## Performance Considerations

When contributing, consider:
- Database query efficiency
- Async/await patterns
- Memory usage
- API response times
- Connection pooling

## Documentation Standards

- Markdown files must be clear and complete
- Code examples must be tested
- API documentation must include request/response examples
- Use clear headers and formatting
- Include troubleshooting sections where applicable

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Questions?

- Check existing issues and PRs
- Review documentation in README.md and DOCKER.md
- Open a discussion issue
- Contact the maintainers

## Code of Conduct

This project follows the Rust Code of Conduct. By participating, you agree to:
- Be respectful to others
- Provide constructive feedback
- Accept criticism gracefully
- Focus on the best outcome for the project

Thank you for contributing! 🚀
