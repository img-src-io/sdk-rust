# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Auto-generated Rust SDK for the [img-src](https://img-src.io) image processing and delivery API. Generated from the OpenAPI specification using [OpenAPI Generator v7.19.0](https://openapi-generator.tech). Direct code changes to generated files will be overwritten on regeneration.

## Commands

```bash
cargo build                  # Build the library
cargo test                   # Run tests
cargo fmt --check            # Check formatting
cargo clippy -- -W warnings  # Lint (warnings as errors in CI)
cargo doc --open             # Generate and view API docs
```

### CI Checks (all must pass)

```bash
cargo fmt --check && cargo clippy -- -W warnings && cargo build && cargo test
```

Note: CI runs `cargo fmt --check` and `cargo clippy` with `continue-on-error: true`, but `cargo build` and `cargo test` must pass.

## Architecture

```
src/
├── lib.rs              # Crate root: re-exports apis and models modules
├── apis/
│   ├── mod.rs          # Error types (Error<T>, ResponseContent<T>), urlencode, deep object parsing
│   ├── configuration.rs # Configuration struct (base_path, client, auth fields)
│   ├── images_api.rs   # Image CRUD, signed URLs, path deletion
│   ├── presets_api.rs  # Transformation preset CRUD (Pro)
│   ├── settings_api.rs # User settings get/update
│   └── usage_api.rs    # Usage statistics
└── models/
    ├── mod.rs          # Re-exports all 30 model structs
    └── *.rs            # Serde-derived request/response structs
```

All API functions are async, take `&Configuration` as first argument, and return `Result<T, Error<SpecificError>>`. Each API module defines per-operation error enums mapping HTTP status codes to typed `ErrorResponse` variants.

### Key Patterns

- **Auth**: Bearer token via `config.bearer_access_token` (API keys with `imgsrc_` prefix)
- **Base URL**: Defaults to `https://api.img-src.io`
- **HTTP client**: `reqwest` with async/await, `tokio` runtime
- **File upload**: Multipart form via `reqwest::multipart::Form` with `tokio::fs::File`
- **TLS features**: `native-tls` (default) or `rustls-tls`

## Code Generation

- Generator: OpenAPI Generator v7.19.0 (version tracked in `.openapi-generator/VERSION`)
- Generated file manifest: `.openapi-generator/FILES`
- Files protected from regeneration: `.openapi-generator-ignore`
- Source spec: `api/openapi.json` in the parent img-src repository
- API changes should be made to the OpenAPI spec, not directly to this SDK

### Post-Generation Fix: Required-Nullable Fields

OpenAPI Generator adds `skip_serializing_if = "Option::is_none"` to all `Option<T>` fields. For fields marked `required: true, nullable: true` in the spec, this is incorrect — they must serialize as `null`, not be omitted. After regeneration, remove `skip_serializing_if` from these fields:

- `src/models/plan_limits.rs` — all 5 fields
- `src/models/preset.rs` — `description`
- `src/models/usage_response.rs` — `subscription_ends_at`

## Deployment

### Publishing to crates.io

1. Update `version` in `Cargo.toml`
2. Commit: `git commit -am "Bump version to X.Y.Z"`
3. Tag and push: `git tag vX.Y.Z && git push origin main --tags`
4. CD workflow (`.github/workflows/cd.yml`) auto-publishes to crates.io

The CD workflow verifies the tag matches `Cargo.toml` version, runs build + unit tests, then publishes. Requires `CARGO_REGISTRY_TOKEN` GitHub secret (from https://crates.io/settings/tokens).

## Testing

### Unit Tests

```bash
cargo test --test models_test --test apis_test
```

- `tests/models_test.rs` — 80 tests: serialization round-trips, realistic API responses, boundary values, missing required fields
- `tests/apis_test.rs` — 36 tests: Configuration, urlencode, parse_deep_object, Error types, API error enum deserialization

### Integration Tests

Full end-to-end test against the live API. Marked `#[ignore]` to avoid CI failures.

```bash
IMGSRC_API_KEY=imgsrc_... cargo test --test integration_test -- --ignored --nocapture
```

Requires a valid API key. Covers: upload, list, search, get, update visibility, delete, settings, usage, presets (Pro), signed URLs (Pro).
