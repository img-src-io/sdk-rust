# img-src Rust SDK

Developer-friendly & type-safe Rust SDK specifically catered to leverage *img-src* API.

[![Crates.io](https://img.shields.io/crates/v/img_src?style=for-the-badge&logo=rust&logoColor=white&label=crates.io&color=e6590a&labelColor=2b2d42)](https://crates.io/crates/img_src)
[![docs.rs](https://img.shields.io/docsrs/img_src?style=for-the-badge&logo=docs.rs&logoColor=white&label=docs.rs&color=1a73e8&labelColor=2b2d42)](https://docs.rs/img_src)
[![License: MIT](https://img.shields.io/badge/LICENSE_//_MIT-3b5bdb?style=for-the-badge&labelColor=eff6ff)](https://opensource.org/licenses/MIT)

<br /><br />

<!-- Start Summary [summary] -->
## Summary

img-src API: Image processing and delivery API.

A serverless image processing and delivery API built on Cloudflare Workers with parameter-driven image transformation and on-demand transcoding.

## Features

- **Image Upload**: Store original images in R2 with SHA256-based deduplication
- **On-Demand Transformation**: Resize, crop, and convert images via URL parameters
- **Format Conversion**: WebP, AVIF, JPEG, PNG output formats
- **Path Organization**: Organize images into folders with multiple paths per image
- **CDN Caching**: Automatic edge caching for transformed images

## Authentication

Authenticate using API Keys with `imgsrc_` prefix. Create your API key at https://img-src.io/settings

## Rate Limiting

- **Free Plan**: 100 requests/minute
- **Pro Plan**: 500 requests/minute

Rate limit headers are included in all responses.
<!-- End Summary [summary] -->

<!-- Start Table of Contents [toc] -->
## Table of Contents
<!-- $toc-max-depth=2 -->
* [img-src Rust SDK](#img-src-rust-sdk)
  * [Features](#features)
  * [Authentication](#authentication)
  * [Rate Limiting](#rate-limiting)
  * [SDK Installation](#sdk-installation)
  * [SDK Example Usage](#sdk-example-usage)
  * [Authentication](#authentication-1)
  * [Available Resources and Operations](#available-resources-and-operations)
  * [Error Handling](#error-handling)
  * [Server Selection](#server-selection)
  * [Custom HTTP Client](#custom-http-client)
* [Development](#development)
  * [Maturity](#maturity)
  * [Contributions](#contributions)

<!-- End Table of Contents [toc] -->

<!-- Start SDK Installation [installation] -->
## SDK Installation

Add the SDK as a dependency to your `Cargo.toml`:

```toml
[dependencies]
img_src = "0.2.0"
```

Or install via cargo:

```bash
cargo add img_src
```
<!-- End SDK Installation [installation] -->

<!-- Start SDK Example Usage [usage] -->
## SDK Example Usage

### Upload and Transform Images

```rust
use img_src::apis::configuration::Configuration;
use img_src::apis::images_api;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create API key at https://img-src.io/settings
    let mut config = Configuration::new();
    config.bearer_access_token = Some(env::var("IMGSRC_API_KEY")?);

    // List images
    let images = images_api::list_images(&config, Some(20), None, None).await?;
    println!("Total: {:?} images", images.total);

    // Access with transformations via CDN
    // https://img-src.io/i/{username}/photos/2024/photo.webp?w=800&h=600&fit=cover&q=85

    Ok(())
}
```
<!-- End SDK Example Usage [usage] -->

<!-- Start Authentication [security] -->
## Authentication

### Per-Client Security Schemes

This SDK supports the following security scheme globally:

| Name         | Type | Scheme      |
| ------------ | ---- | ----------- |
| `BearerAuth` | http | HTTP Bearer |

You can configure it by setting the `bearer_access_token` field in the `Configuration` struct. For example:

```rust
use img_src::apis::configuration::Configuration;
use img_src::apis::settings_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Configuration::new();
    config.bearer_access_token = Some(env::var("IMGSRC_API_KEY")?);

    let settings = settings_api::get_settings(&config).await?;
    println!("{:?}", settings);

    Ok(())
}
```

Create your API key at [https://img-src.io/settings](https://img-src.io/settings).
<!-- End Authentication [security] -->

<!-- Start Available Resources and Operations [operations] -->
## Available Resources and Operations

<details open>
<summary>Available methods</summary>

### [Images](docs/ImagesApi.md)

* [upload_image](docs/ImagesApi.md#upload_image) - Upload image
* [list_images](docs/ImagesApi.md#list_images) - List images
* [search_images](docs/ImagesApi.md#search_images) - Search images
* [get_image](docs/ImagesApi.md#get_image) - Get image metadata
* [delete_image](docs/ImagesApi.md#delete_image) - Delete image
* [create_signed_url](docs/ImagesApi.md#create_signed_url) - Create signed URL
* [delete_image_path](docs/ImagesApi.md#delete_image_path) - Delete image path

### [Presets](docs/PresetsApi.md)

* [list_presets](docs/PresetsApi.md#list_presets) - List presets
* [create_preset](docs/PresetsApi.md#create_preset) - Create preset
* [get_preset](docs/PresetsApi.md#get_preset) - Get preset
* [update_preset](docs/PresetsApi.md#update_preset) - Update preset
* [delete_preset](docs/PresetsApi.md#delete_preset) - Delete preset

### [Settings](docs/SettingsApi.md)

* [get_settings](docs/SettingsApi.md#get_settings) - Get user settings
* [update_settings](docs/SettingsApi.md#update_settings) - Update user settings

### [Usage](docs/UsageApi.md)

* [get_usage](docs/UsageApi.md#get_usage) - Get usage statistics

</details>
<!-- End Available Resources and Operations [operations] -->

<!-- Start Error Handling [errors] -->
## Error Handling

Handling errors in this SDK should largely match your expectations. All operations return a `Result` type, they will never panic.

By default, an API error will return `Error<T>`. You can handle errors using pattern matching:

```rust
use img_src::apis::configuration::Configuration;
use img_src::apis::settings_api;

#[tokio::main]
async fn main() {
    let mut config = Configuration::new();
    config.bearer_access_token = Some("your_api_key".to_string());

    match settings_api::get_settings(&config).await {
        Ok(settings) => println!("Settings: {:?}", settings),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
```
<!-- End Error Handling [errors] -->

<!-- Start Server Selection [server] -->
## Server Selection

### Override Server URL Per-Client

The default server can be overridden globally by setting the `base_path` field in the `Configuration` struct. For example:

```rust
use img_src::apis::configuration::Configuration;

let mut config = Configuration::new();
config.base_path = "https://api.img-src.io".to_string();
config.bearer_access_token = Some("your_api_key".to_string());
```
<!-- End Server Selection [server] -->

<!-- Start Custom HTTP Client [http-client] -->
## Custom HTTP Client

The Rust SDK makes API calls using [reqwest](https://docs.rs/reqwest/). You can customize the HTTP client by providing your own `reqwest::Client` instance:

```rust
use img_src::apis::configuration::Configuration;
use reqwest::Client;
use std::time::Duration;

let http_client = Client::builder()
    .timeout(Duration::from_secs(30))
    .build()
    .unwrap();

let mut config = Configuration::new();
config.client = http_client;
config.bearer_access_token = Some("your_api_key".to_string());
```

This can be a convenient way to configure timeouts, proxies, custom headers, and other low-level configuration.
<!-- End Custom HTTP Client [http-client] -->

## Documentation For Models

- [AvailableFormats](docs/AvailableFormats.md)
- [CdnUrls](docs/CdnUrls.md)
- [CreatePresetRequest](docs/CreatePresetRequest.md)
- [CreateSignedUrlRequest](docs/CreateSignedUrlRequest.md)
- [CreateSignedUrlRequestTransformation](docs/CreateSignedUrlRequestTransformation.md)
- [CurrentPeriod](docs/CurrentPeriod.md)
- [DeletePresetResponse](docs/DeletePresetResponse.md)
- [DeleteResponse](docs/DeleteResponse.md)
- [ErrorDetail](docs/ErrorDetail.md)
- [ErrorResponse](docs/ErrorResponse.md)
- [FolderItem](docs/FolderItem.md)
- [HateoasLinks](docs/HateoasLinks.md)
- [ImageDimensions](docs/ImageDimensions.md)
- [ImageListItem](docs/ImageListItem.md)
- [ImageListResponse](docs/ImageListResponse.md)
- [ImageMetadata](docs/ImageMetadata.md)
- [ListPresetsResponse](docs/ListPresetsResponse.md)
- [MetadataResponse](docs/MetadataResponse.md)
- [PathDeleteResponse](docs/PathDeleteResponse.md)
- [PlanLimits](docs/PlanLimits.md)
- [Preset](docs/Preset.md)
- [SearchResponse](docs/SearchResponse.md)
- [SearchResult](docs/SearchResult.md)
- [SettingsResponse](docs/SettingsResponse.md)
- [SettingsUpdateResponse](docs/SettingsUpdateResponse.md)
- [SignedUrlResponse](docs/SignedUrlResponse.md)
- [UpdatePresetRequest](docs/UpdatePresetRequest.md)
- [UpdateSettingsRequest](docs/UpdateSettingsRequest.md)
- [UploadResponse](docs/UploadResponse.md)
- [UsageResponse](docs/UsageResponse.md)
- [UserSettings](docs/UserSettings.md)


To get access to the crate's generated documentation, use:

```bash
cargo doc --open
```

# Development

## Maturity

This SDK is in beta, and there may be breaking changes between versions without a major version update. Therefore, we recommend pinning usage to a specific package version. This way, you can install the same version each time without breaking changes unless you are intentionally looking for the latest version.

## Contributions

While we value open-source contributions to this SDK, this library is generated programmatically. Any manual changes added to internal files will be overwritten on the next generation. We look forward to hearing your feedback. Feel free to open a PR or an issue with a proof of concept and we'll do our best to include it in a future release.

## Author

[![Taehun](https://github.com/Taehun.png?size=50)](https://github.com/Taehun)

For more information, please visit [https://docs.img-src.io](https://docs.img-src.io)
