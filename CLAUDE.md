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
