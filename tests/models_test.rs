use img_src::models::*;
use serde_json::json;
use std::collections::HashMap;

// ============================================================
// Helper: serialize → deserialize round-trip assertion
// ============================================================
fn round_trip<T>(val: &T)
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let json = serde_json::to_string(val).expect("serialize");
    let back: T = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(*val, back);
}

// ============================================================
// New models (added in spec alignment)
// ============================================================

#[test]
fn active_signed_url_serde_round_trip() {
    let m = ActiveSignedUrl::new(
        "https://cdn.img-src.io/signed/abc?sig=xyz".into(),
        1700000000,
    );
    round_trip(&m);
}

#[test]
fn active_signed_url_deserialize_from_json() {
    let json = json!({
        "signed_url": "https://cdn.img-src.io/signed/abc",
        "expires_at": 1700000000_i64
    });
    let m: ActiveSignedUrl = serde_json::from_value(json).unwrap();
    assert_eq!(m.signed_url, "https://cdn.img-src.io/signed/abc");
    assert_eq!(m.expires_at, 1700000000);
}

#[test]
fn credits_serde_round_trip() {
    let m = Credits::new(5_368_709_120, 100_000, 50_000);
    round_trip(&m);
}

#[test]
fn credits_large_values() {
    let json = json!({
        "storage_bytes": 10_737_418_240_i64,
        "api_requests": 999_999_999_i64,
        "transformations": 500_000_000_i64
    });
    let m: Credits = serde_json::from_value(json).unwrap();
    assert_eq!(m.storage_bytes, 10_737_418_240);
}

#[test]
fn update_visibility_request_serde_round_trip() {
    let m = UpdateVisibilityRequest::new("private".into());
    round_trip(&m);
    assert_eq!(m.visibility, "private");
}

#[test]
fn update_visibility_response_serde_round_trip() {
    let m = UpdateVisibilityResponse::new(
        "abc123".into(),
        "public".into(),
        "Visibility updated".into(),
    );
    round_trip(&m);
}

// ============================================================
// Modified models — type changes and new fields
// ============================================================

#[test]
fn upload_response_has_visibility_and_i64_size() {
    let json = json!({
        "id": "abc123",
        "hash": "deadbeef",
        "url": "https://cdn.img-src.io/user/photo.webp",
        "paths": ["user/photo.webp"],
        "is_new": true,
        "visibility": "public",
        "size": 5_368_709_120_i64,
        "format": "webp",
        "available_formats": { "webp": "u.webp", "avif": "u.avif", "jpeg": "u.jpeg" },
        "uploaded_at": "2024-01-01T00:00:00Z",
        "_links": { "self": "/api/v1/images/abc123", "delete": "/api/v1/images/abc123" }
    });
    let m: UploadResponse = serde_json::from_value(json).unwrap();
    assert_eq!(m.visibility, "public");
    assert_eq!(m.size, 5_368_709_120);
    assert_eq!(m.is_new, Some(true));
}

#[test]
fn upload_response_constructor() {
    let m = UploadResponse::new(
        "id".into(),
        "hash".into(),
        "url".into(),
        vec!["p".into()],
        "public".into(),
        1024,
        "webp".into(),
        AvailableFormats::new("w".into(), "a".into(), "j".into()),
        "2024-01-01".into(),
        HateoasLinks::new("/self".into(), "/del".into()),
    );
    assert_eq!(m.visibility, "public");
    assert_eq!(m.size, 1024);
    assert!(m.is_new.is_none());
    assert!(m.dimensions.is_none());
    round_trip(&m);
}

#[test]
fn image_list_item_has_visibility_and_active_signed_url() {
    let json = json!({
        "id": "img1",
        "original_filename": "photo.png",
        "sanitized_filename": "photo.png",
        "visibility": "private",
        "size": 2_147_483_648_i64,
        "uploaded_at": "2024-06-01T12:00:00Z",
        "url": "/api/v1/images/img1",
        "cdn_url": "https://cdn.img-src.io/user/photo.png",
        "paths": ["user/photo.png"],
        "active_signed_url": {
            "signed_url": "https://cdn.img-src.io/signed/img1",
            "expires_at": 1700000000
        }
    });
    let m: ImageListItem = serde_json::from_value(json).unwrap();
    assert_eq!(m.visibility, "private");
    assert_eq!(m.size, 2_147_483_648);
    let signed = m.active_signed_url.as_ref().unwrap();
    assert_eq!(signed.expires_at, 1700000000);
}

#[test]
fn image_list_item_without_optional_fields() {
    let json = json!({
        "id": "img1",
        "original_filename": "photo.png",
        "visibility": "public",
        "size": 1024,
        "uploaded_at": "2024-01-01",
        "url": "/api",
        "paths": ["p"]
    });
    let m: ImageListItem = serde_json::from_value(json).unwrap();
    assert!(m.sanitized_filename.is_none());
    assert!(m.cdn_url.is_none());
    assert!(m.active_signed_url.is_none());
}

#[test]
fn image_list_item_constructor() {
    let m = ImageListItem::new(
        "id".into(),
        "file.png".into(),
        "public".into(),
        2048,
        "2024-01-01".into(),
        "/api".into(),
        vec!["p".into()],
    );
    assert_eq!(m.visibility, "public");
    assert!(m.active_signed_url.is_none());
    round_trip(&m);
}

#[test]
fn search_result_has_visibility_and_i64_size() {
    let json = json!({
        "id": "s1",
        "original_filename": "test.jpg",
        "paths": ["u/test.jpg"],
        "visibility": "public",
        "size": 3_000_000_000_i64,
        "uploaded_at": "2024-01-01",
        "url": "/api/v1/images/s1"
    });
    let m: SearchResult = serde_json::from_value(json).unwrap();
    assert_eq!(m.visibility, "public");
    assert_eq!(m.size, 3_000_000_000);
}

#[test]
fn search_result_constructor() {
    let m = SearchResult::new(
        "id".into(),
        "f.jpg".into(),
        vec!["p".into()],
        "private".into(),
        4096,
        "2024-01-01".into(),
        "/api".into(),
    );
    assert_eq!(m.visibility, "private");
    assert_eq!(m.size, 4096);
    round_trip(&m);
}

#[test]
fn metadata_response_has_visibility() {
    let json = json!({
        "id": "m1",
        "visibility": "private",
        "metadata": {
            "hash": "deadbeef",
            "original_filename": "img.png",
            "size": 5_000_000_000_i64,
            "uploaded_at": "2024-01-01",
            "mime_type": "image/png"
        },
        "urls": {
            "original": "o", "webp": "w", "avif": "a", "jpeg": "j", "png": "p"
        },
        "_links": { "self": "/s", "delete": "/d" }
    });
    let m: MetadataResponse = serde_json::from_value(json).unwrap();
    assert_eq!(m.visibility, "private");
    assert_eq!(m.metadata.size, 5_000_000_000);
}

#[test]
fn metadata_response_constructor() {
    let m = MetadataResponse::new(
        "id".into(),
        "public".into(),
        ImageMetadata::new("h".into(), "f".into(), 1024, "t".into(), "image/png".into()),
        CdnUrls::new("o".into(), "w".into(), "a".into(), "j".into(), "p".into()),
        HateoasLinks::new("/s".into(), "/d".into()),
    );
    assert_eq!(m.visibility, "public");
    round_trip(&m);
}

#[test]
fn user_settings_has_plan_and_i64_timestamps() {
    let json = json!({
        "id": "user_123",
        "username": "john",
        "email": "john@example.com",
        "plan": "pro",
        "delivery_formats": ["webp", "avif"],
        "default_quality": 85,
        "default_fit_mode": "cover",
        "default_max_width": 1920,
        "default_max_height": null,
        "theme": "dark",
        "language": "en",
        "created_at": 1700000000_i64,
        "updated_at": 1700100000_i64,
        "total_uploads": 42,
        "storage_used_bytes": 5_368_709_120_i64
    });
    let m: UserSettings = serde_json::from_value(json).unwrap();
    assert_eq!(m.plan, "pro");
    assert_eq!(m.created_at, 1700000000);
    assert_eq!(m.updated_at, 1700100000);
    assert_eq!(m.storage_used_bytes, 5_368_709_120);
    assert_eq!(m.email, Some("john@example.com".into()));
    assert_eq!(m.default_max_width, Some(1920));
    assert_eq!(m.default_max_height, None);
}

#[test]
fn user_settings_constructor() {
    let m = UserSettings::new(
        "id".into(),
        "user".into(),
        "free".into(),
        vec!["webp".into()],
        80,
        "cover".into(),
        "light".into(),
        "en".into(),
        1700000000,
        1700000000,
        10,
        1024,
    );
    assert_eq!(m.plan, "free");
    assert!(m.email.is_none());
    round_trip(&m);
}

#[test]
fn usage_response_nullable_subscription_ends_at_and_credits() {
    let json = json!({
        "plan": "pro",
        "plan_name": "Pro Plan",
        "plan_status": "active",
        "subscription_ends_at": null,
        "plan_limits": {
            "max_uploads_per_month": 10000,
            "max_storage_bytes": 10_737_418_240_i64,
            "max_bandwidth_per_month": null,
            "max_api_requests_per_month": null,
            "max_transformations_per_month": null
        },
        "total_images": 100,
        "storage_used_bytes": 5_368_709_120_i64,
        "storage_used_mb": 5120.0,
        "storage_used_gb": 5.0,
        "current_period": {
            "period": "2024-01",
            "period_start": 1704067200_i64,
            "period_end": 1706745600_i64,
            "uploads": 15,
            "bandwidth_bytes": 1_073_741_824_i64,
            "api_requests": 500,
            "transformations": 200
        },
        "credits": {
            "storage_bytes": 5_368_709_120_i64,
            "api_requests": 100000,
            "transformations": 50000
        }
    });
    let m: UsageResponse = serde_json::from_value(json).unwrap();
    assert_eq!(m.subscription_ends_at, None);
    assert_eq!(m.storage_used_bytes, 5_368_709_120);
    assert_eq!(m.credits.storage_bytes, 5_368_709_120);
    assert_eq!(m.plan_limits.max_uploads_per_month, Some(10000));
    assert_eq!(m.plan_limits.max_bandwidth_per_month, None);
    assert_eq!(m.current_period.period_start, 1704067200);
    assert_eq!(m.current_period.bandwidth_bytes, 1_073_741_824);
}

#[test]
fn usage_response_with_subscription_ends_at() {
    let json = json!({
        "plan": "pro",
        "plan_name": "Pro Plan",
        "plan_status": "cancelling",
        "subscription_ends_at": 1706745600_i64,
        "plan_limits": {},
        "total_images": 0,
        "storage_used_bytes": 0,
        "storage_used_mb": 0.0,
        "storage_used_gb": 0.0,
        "current_period": {
            "period": "2024-01",
            "period_start": 1704067200_i64,
            "period_end": 1706745600_i64,
            "uploads": 0,
            "bandwidth_bytes": 0,
            "api_requests": 0,
            "transformations": 0
        },
        "credits": {
            "storage_bytes": 0,
            "api_requests": 0,
            "transformations": 0
        }
    });
    let m: UsageResponse = serde_json::from_value(json).unwrap();
    assert_eq!(m.subscription_ends_at, Some(1706745600));
}

#[test]
fn usage_response_constructor() {
    let m = UsageResponse::new(
        "free".into(),
        "Free Plan".into(),
        usage_response::PlanStatus::Active,
        PlanLimits::new(),
        10,
        2048,
        0.002,
        0.0,
        CurrentPeriod::new("2024-01".into(), 1704067200, 1706745600, 5, 1024, 100, 50),
        Credits::new(1024, 100, 50),
    );
    assert!(m.subscription_ends_at.is_none());
    assert_eq!(m.credits.api_requests, 100);
    round_trip(&m);
}

#[test]
fn plan_status_serde() {
    let active = usage_response::PlanStatus::Active;
    let json = serde_json::to_string(&active).unwrap();
    assert_eq!(json, "\"active\"");

    let cancelling: usage_response::PlanStatus = serde_json::from_str("\"cancelling\"").unwrap();
    assert_eq!(cancelling, usage_response::PlanStatus::Cancelling);

    let expired: usage_response::PlanStatus = serde_json::from_str("\"expired\"").unwrap();
    assert_eq!(expired, usage_response::PlanStatus::Expired);
}

#[test]
fn plan_limits_all_nullable() {
    let json = json!({
        "max_uploads_per_month": null,
        "max_storage_bytes": null,
        "max_bandwidth_per_month": null,
        "max_api_requests_per_month": null,
        "max_transformations_per_month": null
    });
    let m: PlanLimits = serde_json::from_value(json).unwrap();
    assert!(m.max_uploads_per_month.is_none());
    assert!(m.max_storage_bytes.is_none());
    assert!(m.max_bandwidth_per_month.is_none());
    assert!(m.max_api_requests_per_month.is_none());
    assert!(m.max_transformations_per_month.is_none());
}

#[test]
fn plan_limits_with_large_values() {
    let json = json!({
        "max_uploads_per_month": 100_000,
        "max_storage_bytes": 10_737_418_240_i64,
        "max_bandwidth_per_month": 5_368_709_120_i64,
        "max_api_requests_per_month": 1_000_000,
        "max_transformations_per_month": 500_000
    });
    let m: PlanLimits = serde_json::from_value(json).unwrap();
    assert_eq!(m.max_storage_bytes, Some(10_737_418_240));
    assert_eq!(m.max_bandwidth_per_month, Some(5_368_709_120));
}

#[test]
fn plan_limits_default_and_constructor() {
    let m = PlanLimits::new();
    assert!(m.max_uploads_per_month.is_none());
    assert!(m.max_storage_bytes.is_none());
    round_trip(&m);
}

#[test]
fn plan_limits_omitted_fields_deserialize_as_none() {
    let json = json!({});
    let m: PlanLimits = serde_json::from_value(json).unwrap();
    assert!(m.max_uploads_per_month.is_none());
}

#[test]
fn plan_limits_skip_serializing_none() {
    let m = PlanLimits::new();
    let json = serde_json::to_value(&m).unwrap();
    assert!(json.as_object().unwrap().is_empty());
}

#[test]
fn current_period_i64_timestamps_and_bandwidth() {
    let m = CurrentPeriod::new(
        "2024-06".into(),
        1717200000,
        1719792000,
        100,
        10_737_418_240,
        5000,
        2000,
    );
    assert_eq!(m.period_start, 1717200000);
    assert_eq!(m.period_end, 1719792000);
    assert_eq!(m.bandwidth_bytes, 10_737_418_240);
    assert_eq!(m.uploads, 100);
    round_trip(&m);
}

#[test]
fn preset_nullable_description_and_i64_timestamps() {
    let json = json!({
        "id": "preset1",
        "name": "thumbnail",
        "description": null,
        "params": { "w": 200, "h": 200, "fit": "cover" },
        "created_at": 1700000000_i64,
        "updated_at": 1700100000_i64,
        "usage_count": 42
    });
    let m: Preset = serde_json::from_value(json).unwrap();
    assert!(m.description.is_none());
    assert_eq!(m.created_at, 1700000000);
    assert_eq!(m.updated_at, 1700100000);
}

#[test]
fn preset_with_description() {
    let json = json!({
        "id": "p1",
        "name": "banner",
        "description": "Banner crop",
        "params": { "w": 1200 },
        "created_at": 1700000000_i64,
        "updated_at": 1700000000_i64,
        "usage_count": 0
    });
    let m: Preset = serde_json::from_value(json).unwrap();
    assert_eq!(m.description, Some("Banner crop".into()));
}

#[test]
fn preset_constructor_no_description() {
    let mut params = HashMap::new();
    params.insert("w".into(), json!(800));
    let m = Preset::new(
        "id".into(),
        "thumb".into(),
        params,
        1700000000,
        1700000000,
        0,
    );
    assert!(m.description.is_none());
    round_trip(&m);
}

#[test]
fn create_signed_url_request_no_transformation_field() {
    let m = CreateSignedUrlRequest::new();
    let json = serde_json::to_value(&m).unwrap();
    let obj = json.as_object().unwrap();
    assert!(
        !obj.contains_key("transformation"),
        "transformation field should not exist"
    );
    assert!(m.expires_in_seconds.is_none());
}

#[test]
fn create_signed_url_request_with_expiry() {
    let json = json!({ "expires_in_seconds": 7200 });
    let m: CreateSignedUrlRequest = serde_json::from_value(json).unwrap();
    assert_eq!(m.expires_in_seconds, Some(7200));
    round_trip(&m);
}

#[test]
fn signed_url_response_i64_expires_at() {
    let json = json!({
        "signed_url": "https://cdn.img-src.io/signed/x",
        "expires_at": 1700000000_i64,
        "expires_in_seconds": 3600
    });
    let m: SignedUrlResponse = serde_json::from_value(json).unwrap();
    assert_eq!(m.expires_at, 1700000000);
    assert_eq!(m.expires_in_seconds, 3600);
}

#[test]
fn signed_url_response_constructor() {
    let m = SignedUrlResponse::new("url".into(), 1700000000, 3600);
    assert_eq!(m.expires_at, 1700000000_i64);
    round_trip(&m);
}

#[test]
fn image_metadata_i64_size() {
    let json = json!({
        "hash": "deadbeef",
        "original_filename": "big.raw",
        "size": 4_294_967_296_i64,
        "uploaded_at": "2024-01-01",
        "mime_type": "image/raw",
        "width": 8000,
        "height": 6000,
        "dominant_color": "ff5500"
    });
    let m: ImageMetadata = serde_json::from_value(json).unwrap();
    assert_eq!(m.size, 4_294_967_296);
    assert_eq!(m.width, Some(8000));
    assert_eq!(m.dominant_color, Some("ff5500".into()));
}

#[test]
fn image_metadata_constructor() {
    let m = ImageMetadata::new(
        "h".into(),
        "f.png".into(),
        2048,
        "t".into(),
        "image/png".into(),
    );
    assert_eq!(m.size, 2048_i64);
    assert!(m.width.is_none());
    assert!(m.height.is_none());
    assert!(m.dominant_color.is_none());
    round_trip(&m);
}

// ============================================================
// Untouched models — verify they still work
// ============================================================

#[test]
fn available_formats_serde() {
    let m = AvailableFormats::new("w.webp".into(), "a.avif".into(), "j.jpeg".into());
    round_trip(&m);
}

#[test]
fn cdn_urls_serde() {
    let m = CdnUrls::new("o".into(), "w".into(), "a".into(), "j".into(), "p".into());
    round_trip(&m);
}

#[test]
fn hateoas_links_serde() {
    let m = HateoasLinks::new("/self".into(), "/delete".into());
    round_trip(&m);
    let json = serde_json::to_value(&m).unwrap();
    assert_eq!(json["self"], "/self");
    assert_eq!(json["delete"], "/delete");
}

#[test]
fn image_dimensions_serde() {
    let m = ImageDimensions::new(1920, 1080);
    round_trip(&m);
}

#[test]
fn error_detail_serde() {
    let m = ErrorDetail::new("NOT_FOUND".into(), "Image not found".into(), 404);
    round_trip(&m);
    assert!(m.path.is_none());
}

#[test]
fn error_detail_with_path() {
    let json = json!({
        "code": "UNAUTHORIZED",
        "message": "Invalid token",
        "status": 401,
        "path": "/api/v1/images"
    });
    let m: ErrorDetail = serde_json::from_value(json).unwrap();
    assert_eq!(m.path, Some("/api/v1/images".into()));
}

#[test]
fn error_response_serde() {
    let m = ErrorResponse::new(ErrorDetail::new("ERR".into(), "msg".into(), 500));
    round_trip(&m);
}

#[test]
fn folder_item_serde() {
    let m = FolderItem::new("photos".into(), 42);
    round_trip(&m);
}

#[test]
fn delete_response_serde() {
    let m = DeleteResponse::new(true, "Deleted".into(), "2024-01-01T00:00:00Z".into());
    round_trip(&m);
    assert!(m.deleted_paths.is_none());
}

#[test]
fn delete_response_with_paths() {
    let json = json!({
        "success": true,
        "message": "Deleted",
        "deleted_paths": ["user/a.png", "user/b.png"],
        "deleted_at": "2024-01-01"
    });
    let m: DeleteResponse = serde_json::from_value(json).unwrap();
    assert_eq!(m.deleted_paths.as_ref().unwrap().len(), 2);
}

#[test]
fn path_delete_response_serde() {
    let m = PathDeleteResponse::new(
        true,
        "Path removed".into(),
        vec!["user/other.png".into()],
        false,
        "2024-01-01".into(),
    );
    round_trip(&m);
    assert!(!m.image_deleted);
}

#[test]
fn delete_preset_response_serde() {
    let m = DeletePresetResponse::new(true, "Preset deleted".into());
    round_trip(&m);
}

#[test]
fn create_preset_request_serde() {
    let mut params = HashMap::new();
    params.insert("w".into(), json!(800));
    params.insert("q".into(), json!(85));
    let m = CreatePresetRequest::new("thumb".into(), params);
    assert!(m.description.is_none());
    round_trip(&m);
}

#[test]
fn update_preset_request_serde() {
    let m = UpdatePresetRequest::new();
    assert!(m.name.is_none());
    assert!(m.description.is_none());
    assert!(m.params.is_none());
    round_trip(&m);
}

#[test]
fn update_settings_request_serde() {
    let m = UpdateSettingsRequest::new();
    assert!(m.delivery_formats.is_none());
    assert!(m.default_quality.is_none());
    round_trip(&m);
}

#[test]
fn update_settings_request_with_values() {
    let json = json!({
        "delivery_formats": ["webp", "avif"],
        "default_quality": 90,
        "theme": "dark"
    });
    let m: UpdateSettingsRequest = serde_json::from_value(json).unwrap();
    assert_eq!(m.delivery_formats.as_ref().unwrap().len(), 2);
    assert_eq!(m.default_quality, Some(90));
    assert_eq!(m.theme, Some("dark".into()));
}

#[test]
fn settings_response_serde() {
    let settings = UserSettings::new(
        "id".into(),
        "user".into(),
        "free".into(),
        vec!["webp".into()],
        80,
        "cover".into(),
        "light".into(),
        "en".into(),
        1700000000,
        1700000000,
        0,
        0,
    );
    let m = SettingsResponse::new(settings);
    round_trip(&m);
}

#[test]
fn settings_update_response_serde() {
    let settings = UserSettings::new(
        "id".into(),
        "user".into(),
        "pro".into(),
        vec!["webp".into()],
        85,
        "contain".into(),
        "dark".into(),
        "ko".into(),
        1700000000,
        1700100000,
        5,
        1024,
    );
    let m = SettingsUpdateResponse::new(settings, "Settings updated".into());
    round_trip(&m);
}

#[test]
fn list_presets_response_serde() {
    let m = ListPresetsResponse::new(vec![], 0);
    round_trip(&m);
}

#[test]
fn search_response_serde() {
    let m = SearchResponse::new(vec![], 0, "test".into());
    round_trip(&m);
}

#[test]
fn image_list_response_serde() {
    let m = ImageListResponse::new(vec![], vec![], 0, 20, 0, false);
    assert!(m.path_filter.is_none());
    round_trip(&m);
}

#[test]
fn image_list_response_with_path_filter() {
    let json = json!({
        "images": [],
        "folders": [],
        "total": 0,
        "limit": 20,
        "offset": 0,
        "has_more": false,
        "path_filter": "photos/2024"
    });
    let m: ImageListResponse = serde_json::from_value(json).unwrap();
    assert_eq!(m.path_filter, Some("photos/2024".into()));
}

// ============================================================
// Default trait
// ============================================================

#[test]
fn models_implement_default() {
    let _: ActiveSignedUrl = Default::default();
    let _: Credits = Default::default();
    let _: UpdateVisibilityRequest = Default::default();
    let _: UpdateVisibilityResponse = Default::default();
    let _: PlanLimits = Default::default();
    let _: CurrentPeriod = Default::default();
    let _: Preset = Default::default();
    let _: CreateSignedUrlRequest = Default::default();
    let _: SignedUrlResponse = Default::default();
    let _: ImageMetadata = Default::default();
    let _: UploadResponse = Default::default();
    let _: ImageListItem = Default::default();
    let _: SearchResult = Default::default();
    let _: MetadataResponse = Default::default();
    let _: UserSettings = Default::default();
    let _: UsageResponse = Default::default();
    let _: ErrorDetail = Default::default();
    let _: ErrorResponse = Default::default();
    let _: FolderItem = Default::default();
}

// ============================================================
// Clone trait
// ============================================================

#[test]
fn models_implement_clone() {
    let original = Credits::new(1024, 100, 50);
    let cloned = original.clone();
    assert_eq!(original, cloned);

    let original = UserSettings::new(
        "id".into(),
        "u".into(),
        "free".into(),
        vec![],
        80,
        "cover".into(),
        "light".into(),
        "en".into(),
        0,
        0,
        0,
        0,
    );
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

// ============================================================
// Edge cases: empty/minimal JSON payloads
// ============================================================

#[test]
fn upload_response_minimal_required_fields() {
    let json = json!({
        "id": "",
        "hash": "",
        "url": "",
        "paths": [],
        "visibility": "public",
        "size": 0,
        "format": "",
        "available_formats": { "webp": "", "avif": "", "jpeg": "" },
        "uploaded_at": "",
        "_links": { "self": "", "delete": "" }
    });
    let m: UploadResponse = serde_json::from_value(json).unwrap();
    assert_eq!(m.size, 0);
    assert!(m.is_new.is_none());
}

#[test]
fn usage_response_plan_status_default() {
    let default = usage_response::PlanStatus::default();
    assert_eq!(default, usage_response::PlanStatus::Active);
}

// ============================================================
// Serialization: verify skip_serializing_if for Option fields
// ============================================================

#[test]
fn optional_fields_omitted_when_none() {
    let m = ImageListItem::new(
        "id".into(),
        "f".into(),
        "public".into(),
        0,
        "t".into(),
        "u".into(),
        vec![],
    );
    let json = serde_json::to_value(&m).unwrap();
    let obj = json.as_object().unwrap();
    assert!(!obj.contains_key("sanitized_filename"));
    assert!(!obj.contains_key("cdn_url"));
    assert!(!obj.contains_key("active_signed_url"));
}

#[test]
fn preset_description_omitted_when_none() {
    let m = Preset::new("id".into(), "name".into(), HashMap::new(), 0, 0, 0);
    let json = serde_json::to_value(&m).unwrap();
    assert!(!json.as_object().unwrap().contains_key("description"));
}

#[test]
fn error_detail_path_omitted_when_none() {
    let m = ErrorDetail::new("ERR".into(), "msg".into(), 500);
    let json = serde_json::to_value(&m).unwrap();
    assert!(!json.as_object().unwrap().contains_key("path"));
}
