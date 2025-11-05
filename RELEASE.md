# Release Checklist for Dintero Rust SDK

## Pre-Release

- [ ] All tests pass: `cargo test --all-features`
- [ ] No clippy warnings: `cargo clippy --all-features -- -D warnings`
- [ ] Documentation builds: `cargo doc --all-features --no-deps`
- [ ] All examples run successfully
- [ ] CHANGELOG.md is updated
- [ ] Version numbers are correct in all Cargo.toml files
- [ ] README.md is up to date
- [ ] All changes are committed and pushed

## Publishing Order

The crates must be published in this specific order because `dintero` depends on all the others:

### 1. Feature Crates (no interdependencies)

These can be published in any order, but we publish them sequentially with a delay to allow crates.io to index them:

```bash
cargo publish -p dintero-checkout
# Wait 30 seconds
cargo publish -p dintero-orders  
# Wait 30 seconds
cargo publish -p dintero-payments
# Wait 30 seconds
cargo publish -p dintero-accounts
# Wait 30 seconds
cargo publish -p dintero-loyalty
# Wait 30 seconds
cargo publish -p dintero-insights
# Wait 30 seconds
```

### 2. Main Crate

After all feature crates are published and indexed:

```bash
cargo publish -p dintero
```

## Using the Publish Script

For convenience, use the provided script:

```bash
./publish.sh
```

This script will:
1. Run all tests and checks
2. Publish all feature crates with appropriate delays
3. Publish the main `dintero` crate
4. Provide confirmation of successful publication

## Post-Release

- [ ] Verify the crate on crates.io: https://crates.io/crates/dintero
- [ ] Check documentation on docs.rs: https://docs.rs/dintero
- [ ] Create a Git tag: `git tag -a v0.1.0 -m "Release v0.1.0"`
- [ ] Push the tag: `git push origin v0.1.0`
- [ ] Create a GitHub release with changelog
- [ ] Update any example projects
- [ ] Announce the release

## Version Bumping for Future Releases

When releasing a new version:

1. Update version in `Cargo.toml` for ALL crates (workspace.package.version)
2. Update CHANGELOG.md
3. Commit the version bump
4. Follow the publishing order above
5. Create and push a new Git tag

## Troubleshooting

### "no matching package found" Error

If you get this error when publishing `dintero`, it means one of the feature crates hasn't been indexed by crates.io yet. Wait a few minutes and try again.

### Rate Limiting

If you hit rate limits, wait a few minutes between publishes. The `publish.sh` script includes 30-second delays to avoid this.

### Failed Publication

If a crate fails to publish:
1. Fix the issue
2. Bump the version number (e.g., 0.1.0 -> 0.1.1)
3. Resume publishing from that crate onwards

## User Installation

After successful publication, users can install with:

```toml
[dependencies]
dintero = "0.1.0"
```

Or with specific features:

```toml
[dependencies]
dintero = { version = "0.1.0", features = ["checkout", "orders"] }
```

The feature crates (`dintero-checkout`, `dintero-orders`, etc.) are internal dependencies and users should never add them directly.
