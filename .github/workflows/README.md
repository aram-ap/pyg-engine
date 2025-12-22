# GitHub Actions Workflows

This directory contains CI/CD workflows for pyg-engine.

## Workflows

### tests.yml

Main testing workflow that runs on every push and pull request.

**What it does:**
- Runs Python tests on Ubuntu, macOS, and Windows
- Tests Python versions 3.8 through 3.12
- Builds and runs C++ tests on Ubuntu and macOS
- Runs linting with Ruff
- Generates coverage reports
- Uploads test results and coverage to Codecov

**Triggers:**
- Push to `main` or `develop` branches
- Pull requests to `main` or `develop` branches
- Manual workflow dispatch

**Jobs:**
- `python-tests` - Run Python tests across multiple platforms/versions
- `cpp-tests` - Build and run C++ tests
- `lint` - Check code style with Ruff
- `integration-tests` - Run integration tests
- `test-summary` - Aggregate results from all test jobs

### codeql.yml

Security analysis workflow using GitHub CodeQL.

**What it does:**
- Scans Python and C++ code for security vulnerabilities
- Checks for common security issues
- Analyzes code quality

**Triggers:**
- Push to `main` or `develop` branches
- Pull requests to `main` or `develop` branches
- Weekly schedule (Monday at midnight)

**Languages analyzed:**
- Python
- C++

## Workflow Status

You can check the status of workflows:

- In the GitHub repository under the "Actions" tab
- On pull requests (checks section)
- Using status badges in README.md

## Local Testing

To test locally before pushing:

```bash
# Run all tests
python run_tests.py --build-cpp

# Run linting
ruff check pyg/ tests/

# Run formatting check
ruff format --check pyg/ tests/
```

## Modifying Workflows

When modifying workflows:

1. Test changes on a feature branch first
2. Use workflow_dispatch for manual testing
3. Check workflow syntax with GitHub's workflow validator
4. Monitor workflow runs for errors
5. Update this README if adding new workflows

## Secrets and Variables

Required secrets (set in repository settings):
- `CODECOV_TOKEN` - For uploading coverage reports (optional)

No other secrets are currently required.

## Caching

Workflows use caching to speed up builds:

- `actions/cache` - Caches pip packages
- Python setup action - Caches pip by default
- CMake build artifacts are not cached (rebuild for consistency)

## Troubleshooting

**Workflow fails with "resource not accessible by integration":**
- Check repository permissions in Settings > Actions > General
- Ensure workflows have write permissions if needed

**Python tests fail but pass locally:**
- Check Python version (workflow tests 3.8-3.12)
- Verify all dependencies are in pyproject.toml
- Check for platform-specific issues

**C++ tests fail but pass locally:**
- Check system dependencies are installed correctly
- Verify CMake version compatibility
- Check for platform-specific code issues

**Linting fails but passes locally:**
- Ensure you're using the same Ruff version
- Run `ruff check pyg/ tests/` locally
- Check .ruff.toml configuration

## Performance

Approximate workflow run times:

- Python tests: 5-10 minutes (per platform/version combo)
- C++ tests: 10-15 minutes (per platform/build type)
- Linting: 1-2 minutes
- Integration tests: 5-10 minutes

Total: ~30-40 minutes for all jobs (running in parallel)

## Future Improvements

Potential workflow enhancements:

- [ ] Add deployment workflow for PyPI releases
- [ ] Add benchmark comparison workflow
- [ ] Add documentation build/deploy workflow
- [ ] Add Docker image build workflow
- [ ] Add performance regression testing






