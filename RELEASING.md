# Releasing

This repository uses a focused-first release flow rather than publishing the
`use-stats` facade crate first.

## Current release state

Publish the focused crates before the `use-stats` umbrella crate.

## Current automation

The repository includes the release files and workflows that match this shape:

- `publish-readiness.yml`
- `facade-publish-readiness.yml`
- `release-plz-pr.yml`
- `release-plz-release.yml`

## Initial publish order

For the first public crates.io wave, publish in this order:

1. `use-average`
2. `use-variance`
3. `use-standard-deviation`
4. `use-percentile`
5. `use-z-score`
6. `use-correlation`
7. `use-distribution`
8. `use-stats`

The `use-stats` facade should come last after the focused crates are visible on
crates.io.

## Follow-up release automation

After the initial manual crates.io wave is complete, the repository can use the
`release-plz` workflows for follow-up releases.

### Release PR automation

- Workflow: `Release PR Automation`
- Trigger: pushes to `main` or manual dispatch
- Purpose: opens or updates a release pull request from `release-plz.toml`

### Release publish automation

- Workflow: `Release Publish Automation`
- Trigger: manual dispatch only
- Required input: `post-initial-release = true`
- Required secret: `CARGO_REGISTRY_TOKEN`

The release publish workflow confirms that every focused crate already exists on
crates.io before it runs `release-plz release`.

## Permanent version warning

Published crates.io versions are permanent. Verify the crate metadata,
packaging, and changelog inputs before any real publish.