# GitHub Actions Workflows

This directory contains GitHub Actions workflows for BPSR Logs.

## Active Workflows

### 📦 release.yml (PRIMARY)
**Automated Release Workflow**

Handles the complete release process:
- ✅ Auto-versioning based on commit messages
- ✅ Automatic changelog generation
- ✅ Professional GitHub releases
- ✅ MSI and NSIS installer creation
- ✅ Auto-update JSON generation
- ✅ Code signing support

**Triggers:**
- Push to `main` branch (auto-version and release)
- Manual workflow dispatch (with version bump selection)
- Tags starting with `v*`

**See:** [RELEASE_PROCESS.md](../../RELEASE_PROCESS.md) for detailed documentation

## Deprecated Workflows

### ⚠️ build.yml (DEPRECATED)
Legacy publish workflow. Use `release.yml` instead.

**Note:** This workflow only triggers on `release-legacy` branch and `legacy-v*` tags.

## Workflow Features

### Auto-Versioning
- Analyzes commit messages using Conventional Commits
- Determines appropriate version bump (major/minor/patch)
- Updates version in `tauri.conf.json` and `Cargo.toml`
- Creates git tag automatically

### Changelog Generation
- Generates changelog from commit history
- Formats release notes with sections
- Includes installation instructions
- Highlights features and known issues

### Professional Releases
- Formatted release body with emoji indicators
- Installation instructions
- Known issues list
- Feature highlights
- Community links

### Build & Deploy
- Windows MSI installer
- NSIS installer (alternative)
- Code signing support
- Auto-update JSON generation
- Artifact upload to GitHub Releases

## Configuration

Required secrets in GitHub repository settings:

- `GITHUB_TOKEN` - Automatically provided by GitHub
- `TAURI_SIGNING_PRIVATE_KEY` - For code signing (optional but recommended)
- `BP_TIMER_ENDPOINT` - BP Timer API endpoint (if used)
- `BP_TIMER_API_KEY` - BP Timer API key (if used)

## Usage

### Automatic Release (Recommended)

1. Make changes and commit using conventional commit format:
   ```bash
   git commit -m "feat: add new feature"
   git commit -m "fix: resolve bug"
   git commit -m "feat!: breaking change"
   ```

2. Create PR and merge to `main`

3. Workflow automatically:
   - Determines version bump
   - Updates version files
   - Generates changelog
   - Creates release
   - Builds installers

### Manual Release

1. Go to GitHub Actions
2. Select "Automated Release" workflow
3. Click "Run workflow"
4. Choose version bump type (patch/minor/major)
5. Click "Run workflow" button

## Monitoring

Check workflow status:
- GitHub Actions tab
- Commit status checks
- Release page
- Action summary in workflow run

## Support

For issues with workflows:
1. Check [RELEASE_PROCESS.md](../../RELEASE_PROCESS.md)
2. Review workflow logs in GitHub Actions
3. Create issue in repository

## Version History

- **2025-12**: Automated release workflow (release.yml) introduced
- **2024**: Original publish workflow (build.yml)
