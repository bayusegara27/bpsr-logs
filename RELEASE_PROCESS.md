# Release Process Documentation

This document explains the automated release process for BPSR Logs.

## 🚀 Automated Release Workflow

The project uses GitHub Actions to automatically handle versioning, changelog generation, and releases.

### How It Works

1. **Auto-Versioning**: When changes are merged to `main`, the workflow automatically:
   - Analyzes commit messages to determine version bump type
   - Updates version in `tauri.conf.json` and `Cargo.toml`
   - Generates changelog from commit history
   - Creates a git tag
   - Pushes changes back to main

2. **Auto-Build**: After versioning, the workflow:
   - Builds the application on Windows
   - Creates MSI and NSIS installers
   - Generates update JSON for auto-update functionality
   - Uploads artifacts to GitHub Releases

3. **Auto-Release**: Creates a professional GitHub Release with:
   - Formatted changelog
   - Installation instructions
   - Known issues
   - Feature highlights
   - Download links

## 📝 Commit Message Convention

The workflow uses commit messages to determine version bumps:

### Major Version (x.0.0)
- Breaking changes
- Use `BREAKING CHANGE:` in commit message or `!` after type

```bash
git commit -m "feat!: redesign entire UI"
git commit -m "fix: remove deprecated API

BREAKING CHANGE: Old API endpoints no longer supported"
```

### Minor Version (0.x.0)
- New features
- Use `feat:` or `feature:` prefix

```bash
git commit -m "feat: add dark mode support"
git commit -m "feature: implement user preferences"
```

### Patch Version (0.0.x)
- Bug fixes
- Documentation updates
- Other changes
- Use `fix:`, `docs:`, `chore:`, etc.

```bash
git commit -m "fix: resolve login issue"
git commit -m "docs: update installation guide"
git commit -m "chore: update dependencies"
```

## 🔄 Manual Release Trigger

You can also trigger a release manually:

1. Go to **Actions** → **Automated Release**
2. Click **Run workflow**
3. Select version bump type: `patch`, `minor`, or `major`
4. Click **Run workflow**

## 📦 What Gets Released

Each release includes:

### Installers
- **MSI Installer**: `bpsr-logs_{version}_x64_en-US.msi`
  - Recommended for most users
  - Integrates with Windows installer
  - Supports automatic updates

- **NSIS Installer**: `bpsr-logs_{version}_x64-setup.exe`
  - Alternative installer format
  - Smaller file size

### Update Files
- `latest.json`: Auto-update configuration
- Signature files for secure updates

## 🔐 Code Signing

The release workflow uses `TAURI_SIGNING_PRIVATE_KEY` secret for code signing.

**To enable code signing:**

1. Generate signing key:
```bash
npm install -g @tauri-apps/cli
tauri signer generate
```

2. Add private key to GitHub Secrets:
   - Go to Settings → Secrets and variables → Actions
   - Create secret named `TAURI_SIGNING_PRIVATE_KEY`
   - Paste the private key

3. Commit the public key to the repository

## 📋 Release Checklist

### Before Release
- [ ] All tests passing
- [ ] Frontend builds successfully
- [ ] Rust code compiles without errors
- [ ] Documentation updated
- [ ] Breaking changes documented

### After Automatic Release
- [ ] Verify release created on GitHub
- [ ] Test download links work
- [ ] Verify installer works on clean Windows system
- [ ] Check auto-update JSON is valid
- [ ] Announce release on Discord

## 🐛 Troubleshooting

### Release Fails to Build

1. Check GitHub Actions logs
2. Verify all dependencies are available
3. Ensure secrets are configured correctly

### Version Not Updated

1. Ensure commit messages follow convention
2. Check if commit was merged to `main` branch
3. Verify workflow permissions

### Auto-Update Not Working

1. Check `latest.json` is in release assets
2. Verify signature files are present
3. Ensure `TAURI_SIGNING_PRIVATE_KEY` is configured

## 🎯 Best Practices

1. **Write Clear Commit Messages**
   - Use conventional commit format
   - Include context and reasoning
   - Reference issue numbers

2. **Keep Changes Focused**
   - One feature/fix per PR
   - Easier to review and rollback
   - Better changelog generation

3. **Test Before Merging**
   - Run local builds
   - Test on clean environment
   - Verify all features work

4. **Update Documentation**
   - Keep README.md current
   - Document new features
   - Update troubleshooting guides

## 📚 Additional Resources

- [Tauri Documentation](https://tauri.app/)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Semantic Versioning](https://semver.org/)
- [Conventional Commits](https://www.conventionalcommits.org/)

## 🔗 Related Files

- `.github/workflows/release.yml` - Main release workflow
- `.github/workflows/build.yml` - Legacy build workflow (deprecated)
- `CHANGELOG.md` - Version history
- `src-tauri/tauri.conf.json` - App version configuration
- `src-tauri/Cargo.toml` - Rust package version

## 💡 Tips

- Use draft releases for testing release process
- Keep changelog updated manually for important releases
- Tag hotfixes with `-hotfix` suffix (e.g., `v0.22.2-hotfix`)
- Use pre-release for beta versions
