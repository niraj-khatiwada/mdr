# Packaging & Release Setup Guide

This document describes how to set up secrets and external repos for automated releases.

## GitHub Actions Secrets

Configure these in **GitHub repo → Settings → Secrets and variables → Actions → Secrets**:

| Secret | How to get it | Used by |
|--------|--------------|---------|
| `HOMEBREW_TAP_TOKEN` | GitHub PAT with write access to `CleverCloud/homebrew-misc` | Homebrew formula update |

### Optional Variables

Configure in **GitHub repo → Settings → Secrets and variables → Actions → Variables**:

| Variable | Value | Purpose |
|----------|-------|---------|
| `HOMEBREW_TAP_ENABLED` | `true` | Enable Homebrew tap updates on release |

## Repos to Create

### `CleverCloud/homebrew-misc`

Homebrew tap for Clever Cloud tools.

1. Create the repo `CleverCloud/homebrew-misc` on GitHub
2. Initialize with a `Formula/` directory
3. Users install with: `brew tap CleverCloud/misc && brew install mdr`

## Setting Up Homebrew Tap Token

1. Go to **GitHub Settings → Developer settings → Personal access tokens → Fine-grained tokens**
2. Click **"Generate new token"**
3. Name: `mdr-homebrew`
4. Resource owner: **CleverCloud**
5. Repository access: **Only select** `CleverCloud/homebrew-misc`
6. Permissions: **Contents: Read and write**
7. Copy token → add as `HOMEBREW_TAP_TOKEN` secret in mdr repo

## Creating a Release

```bash
# Tag the release
git tag v0.1.0
git push origin v0.1.0
```

This triggers the release workflow which:
1. Builds binaries for macOS (ARM + Intel), Linux (x86_64), and Windows (x86_64)
2. Builds `.deb` package (Debian/Ubuntu)
3. Builds `.rpm` package (Fedora/RHEL)
4. Creates a GitHub Release with all artifacts
5. Updates Homebrew formula (if enabled)

## Nix Flake

Users can install directly with:

```bash
nix run github:CleverCloud/mdr
```

Or add to a flake:

```nix
{
  inputs.mdr.url = "github:CleverCloud/mdr";
}
```

## Future: crates.io

To publish on crates.io:

1. Go to https://crates.io/settings/tokens
2. "New Token" → name: `mdr-ci` → scope: publish-update → crate: `mdr`
3. Add as `CARGO_REGISTRY_TOKEN` secret
4. Add a publish job to release.yml

## Future: WinGet

Use `vedantmgoyal9/winget-releaser@v2` action with a PAT that has `public_repo` scope.
