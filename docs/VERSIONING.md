# Versioning Guide

## Current Setup: setuptools-scm (Automatic from Git Tags)

Your project now uses **setuptools-scm** which automatically generates version numbers from git tags. This means:
- ✅ **No manual version updates** in code files
- ✅ **Single source of truth** (git tags)
- ✅ **No version mismatches** between files
- ✅ **Automatic dev versions** between releases

## How to Release a New Version

### 1. Make your changes and commit
```bash
git add .
git commit -m "Add new feature"
```

### 2. Create a git tag for the release
```bash
# For a new release (e.g., v1.2.1)
git tag v1.2.1

# Or with an annotated tag (recommended)
git tag -a v1.2.1 -m "Release version 1.2.1"
```

### 3. Push the tag to trigger the release
```bash
git push origin v1.2.1
```

### 4. Create a GitHub release
```bash
gh release create v1.2.1 --title "Version 1.2.1" --notes "Release notes here"
```

**That's it!** The GitHub Action will automatically build and publish to PyPI.

## Version Number Format

Follow [Semantic Versioning](https://semver.org/):
- **MAJOR.MINOR.PATCH** (e.g., 1.2.1)
- **MAJOR**: Breaking changes (v2.0.0)
- **MINOR**: New features, backwards compatible (v1.3.0)
- **PATCH**: Bug fixes (v1.2.1)

## Development Versions

Between releases, setuptools-scm automatically generates dev versions:
- After tagging v1.2.1, the next commit becomes `1.2.2.dev1+g<commit-hash>`
- This helps distinguish release builds from development builds

## Checking the Current Version

```bash
# In Python
import pyg_engine
print(pyg_engine.__version__)

# From command line
python -c "import pyg_engine; print(pyg_engine.__version__)"
```

## Alternative: Manual Versioning (if you prefer)

If you want manual control instead:

1. **Revert to manual versioning:**
   - Edit `pyproject.toml`: change `dynamic = ["version"]` to `version = "1.2.1"`
   - Remove `[tool.setuptools_scm]` section
   - Update version manually before each release

2. **Use bump2version tool:**
   ```bash
   pip install bump2version
   bump2version patch  # 1.2.0 -> 1.2.1
   bump2version minor  # 1.2.1 -> 1.3.0
   bump2version major  # 1.3.0 -> 2.0.0
   ```

## Troubleshooting

**"version is unknown"**
- Make sure you have at least one git tag (e.g., `git tag v1.2.0`)
- Commit your changes first
- The version is generated at build time

**"setuptools-scm not found"**
- Run: `pip install setuptools-scm>=8.0`
