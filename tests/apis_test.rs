use img_src::apis::configuration::{ApiKey, Configuration};
use img_src::apis::{parse_deep_object, urlencode};
use serde_json::json;

// ============================================================
// Configuration
// ============================================================

#[test]
fn configuration_default_base_path() {
    let config = Configuration::default();
    assert_eq!(config.base_path, "https://api.img-src.io");
}

#[test]
fn configuration_default_user_agent() {
    let config = Configuration::default();
    assert_eq!(
        config.user_agent,
        Some("OpenAPI-Generator/1.0.0/rust".to_string())
    );
}

#[test]
fn configuration_default_auth_fields_are_none() {
    let config = Configuration::default();
    assert!(config.basic_auth.is_none());
    assert!(config.oauth_access_token.is_none());
    assert!(config.bearer_access_token.is_none());
    assert!(config.api_key.is_none());
}

#[test]
fn configuration_new_equals_default() {
    let a = Configuration::new();
    let b = Configuration::default();
    assert_eq!(a.base_path, b.base_path);
    assert_eq!(a.user_agent, b.user_agent);
    assert_eq!(a.bearer_access_token, b.bearer_access_token);
}

#[test]
fn configuration_set_bearer_token() {
    let mut config = Configuration::new();
    config.bearer_access_token = Some("imgsrc_test_key_123".into());
    assert_eq!(
        config.bearer_access_token,
        Some("imgsrc_test_key_123".into())
    );
}

#[test]
fn configuration_set_custom_base_path() {
    let mut config = Configuration::new();
    config.base_path = "http://localhost:8787".into();
    assert_eq!(config.base_path, "http://localhost:8787");
}

#[test]
fn configuration_set_api_key() {
    let mut config = Configuration::new();
    config.api_key = Some(ApiKey {
        prefix: Some("Bearer".into()),
        key: "imgsrc_abc123".into(),
    });
    let api_key = config.api_key.unwrap();
    assert_eq!(api_key.prefix, Some("Bearer".into()));
    assert_eq!(api_key.key, "imgsrc_abc123");
}

#[test]
fn configuration_set_basic_auth() {
    let mut config = Configuration::new();
    config.basic_auth = Some(("user".into(), Some("pass".into())));
    let (user, pass) = config.basic_auth.unwrap();
    assert_eq!(user, "user");
    assert_eq!(pass, Some("pass".into()));
}

#[test]
fn configuration_clone() {
    let mut config = Configuration::new();
    config.bearer_access_token = Some("token".into());
    let cloned = config.clone();
    assert_eq!(cloned.bearer_access_token, Some("token".into()));
    assert_eq!(cloned.base_path, config.base_path);
}

// ============================================================
// urlencode
// ============================================================

#[test]
fn urlencode_plain_string() {
    assert_eq!(urlencode("hello"), "hello");
}

#[test]
fn urlencode_spaces() {
    assert_eq!(urlencode("hello world"), "hello+world");
}

#[test]
fn urlencode_special_characters() {
    let encoded = urlencode("path/to/file name.png");
    assert_eq!(encoded, "path%2Fto%2Ffile+name.png");
}

#[test]
fn urlencode_unicode() {
    let encoded = urlencode("이미지");
    assert!(!encoded.is_empty());
    assert!(!encoded.contains('이'));
}

#[test]
fn urlencode_already_safe() {
    assert_eq!(urlencode("abc123"), "abc123");
}

#[test]
fn urlencode_empty_string() {
    assert_eq!(urlencode(""), "");
}

// ============================================================
// parse_deep_object
// ============================================================

#[test]
fn parse_deep_object_flat() {
    let val = json!({ "width": "800", "height": "600" });
    let params = parse_deep_object("filter", &val);
    assert!(params.contains(&("filter[width]".to_string(), "800".to_string())));
    assert!(params.contains(&("filter[height]".to_string(), "600".to_string())));
}

#[test]
fn parse_deep_object_nested() {
    let val = json!({
        "transform": {
            "width": "400"
        }
    });
    let params = parse_deep_object("opts", &val);
    assert!(params.contains(&("opts[transform][width]".to_string(), "400".to_string())));
}

#[test]
fn parse_deep_object_numeric_values() {
    let val = json!({ "quality": 85 });
    let params = parse_deep_object("p", &val);
    assert!(params.contains(&("p[quality]".to_string(), "85".to_string())));
}

#[test]
fn parse_deep_object_array_of_objects() {
    let val = json!({ "items": [{ "name": "a" }, { "name": "b" }] });
    let params = parse_deep_object("p", &val);
    assert!(params.contains(&("p[items][0][name]".to_string(), "a".to_string())));
    assert!(params.contains(&("p[items][1][name]".to_string(), "b".to_string())));
}

#[test]
fn parse_deep_object_boolean() {
    let val = json!({ "enabled": true });
    let params = parse_deep_object("p", &val);
    assert!(params.contains(&("p[enabled]".to_string(), "true".to_string())));
}

// ============================================================
// Error types
// ============================================================

#[test]
fn error_display_serde_via_from() {
    let serde_err = serde_json::from_str::<String>("not json").unwrap_err();
    let api_err: img_src::apis::Error<()> = img_src::apis::Error::from(serde_err);
    let display = format!("{}", api_err);
    assert!(display.contains("serde"));
}

#[test]
fn error_display_serde() {
    let serde_err = serde_json::from_str::<String>("invalid").unwrap_err();
    let err: img_src::apis::Error<()> = img_src::apis::Error::Serde(serde_err);
    let display = format!("{}", err);
    assert!(display.starts_with("error in serde:"));
}

#[test]
fn error_display_io() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let err: img_src::apis::Error<()> = img_src::apis::Error::Io(io_err);
    let display = format!("{}", err);
    assert!(display.starts_with("error in IO:"));
    assert!(display.contains("file not found"));
}

#[test]
fn error_display_response() {
    let err: img_src::apis::Error<String> =
        img_src::apis::Error::ResponseError(img_src::apis::ResponseContent {
            status: reqwest::StatusCode::NOT_FOUND,
            content: "not found".into(),
            entity: Some("entity".into()),
        });
    let display = format!("{}", err);
    assert!(display.contains("404"));
}

#[test]
fn error_from_serde() {
    let serde_err = serde_json::from_str::<String>("bad").unwrap_err();
    let err: img_src::apis::Error<()> = img_src::apis::Error::from(serde_err);
    match err {
        img_src::apis::Error::Serde(_) => {}
        _ => panic!("expected Serde variant"),
    }
}

#[test]
fn error_from_io() {
    let io_err = std::io::Error::new(std::io::ErrorKind::Other, "test");
    let err: img_src::apis::Error<()> = img_src::apis::Error::from(io_err);
    match err {
        img_src::apis::Error::Io(_) => {}
        _ => panic!("expected Io variant"),
    }
}

#[test]
fn response_content_fields() {
    let rc = img_src::apis::ResponseContent::<String> {
        status: reqwest::StatusCode::OK,
        content: "{\"ok\":true}".into(),
        entity: None,
    };
    assert_eq!(rc.status, reqwest::StatusCode::OK);
    assert!(rc.entity.is_none());
}

// ============================================================
// Error enum deserialization (typed API errors)
// ============================================================

#[test]
fn images_api_error_enums_deserialize() {
    use img_src::apis::images_api::*;

    let error_json = json!({
        "error": {
            "code": "UNAUTHORIZED",
            "message": "Invalid token",
            "status": 401
        }
    });

    // All error enums should be able to deserialize ErrorResponse
    let _: CreateSignedUrlError = serde_json::from_value(error_json.clone()).unwrap();
    let _: DeleteImageError = serde_json::from_value(error_json.clone()).unwrap();
    let _: DeleteImagePathError = serde_json::from_value(error_json.clone()).unwrap();
    let _: GetImageError = serde_json::from_value(error_json.clone()).unwrap();
    let _: ListImagesError = serde_json::from_value(error_json.clone()).unwrap();
    let _: SearchImagesError = serde_json::from_value(error_json.clone()).unwrap();
    let _: UploadImageError = serde_json::from_value(error_json.clone()).unwrap();
    let _: UpdateVisibilityError = serde_json::from_value(error_json).unwrap();
}

#[test]
fn presets_api_error_enums_deserialize() {
    use img_src::apis::presets_api::*;

    let error_json = json!({
        "error": {
            "code": "NOT_FOUND",
            "message": "Preset not found",
            "status": 404
        }
    });

    let _: CreatePresetError = serde_json::from_value(error_json.clone()).unwrap();
    let _: DeletePresetError = serde_json::from_value(error_json.clone()).unwrap();
    let _: GetPresetError = serde_json::from_value(error_json.clone()).unwrap();
    let _: ListPresetsError = serde_json::from_value(error_json.clone()).unwrap();
    let _: UpdatePresetError = serde_json::from_value(error_json).unwrap();
}

#[test]
fn settings_api_error_enums_deserialize() {
    use img_src::apis::settings_api::*;

    let error_json = json!({
        "error": {
            "code": "UNAUTHORIZED",
            "message": "Unauthorized",
            "status": 401
        }
    });

    let _: GetSettingsError = serde_json::from_value(error_json.clone()).unwrap();
    let _: UpdateSettingsError = serde_json::from_value(error_json).unwrap();
}

#[test]
fn usage_api_error_enums_deserialize() {
    use img_src::apis::usage_api::*;

    let error_json = json!({
        "error": {
            "code": "INTERNAL_ERROR",
            "message": "Something went wrong",
            "status": 500
        }
    });

    let _: GetUsageError = serde_json::from_value(error_json).unwrap();
}

// ============================================================
// ResponseContent and Error type tests
// ============================================================

#[test]
fn error_response_content_with_json_body() {
    let rc = img_src::apis::ResponseContent::<String> {
        status: reqwest::StatusCode::BAD_REQUEST,
        content: r#"{"error":{"code":"VALIDATION_ERROR","message":"Invalid params","status":400}}"#
            .into(),
        entity: Some("parsed entity".into()),
    };
    assert_eq!(rc.status, reqwest::StatusCode::BAD_REQUEST);
    assert!(rc.content.contains("VALIDATION_ERROR"));
    assert_eq!(rc.entity, Some("parsed entity".into()));
}

#[test]
fn error_response_content_with_text_body() {
    let rc = img_src::apis::ResponseContent::<String> {
        status: reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        content: "Internal Server Error".into(),
        entity: None,
    };
    assert_eq!(rc.status, reqwest::StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(rc.content, "Internal Server Error");
    assert!(rc.entity.is_none());
}

#[test]
fn error_display_response_with_typed_entity() {
    let rc = img_src::apis::ResponseContent {
        status: reqwest::StatusCode::FORBIDDEN,
        content: "forbidden".into(),
        entity: Some(
            json!({"error": {"code": "FORBIDDEN", "message": "Access denied", "status": 403}}),
        ),
    };
    let err: img_src::apis::Error<serde_json::Value> = img_src::apis::Error::ResponseError(rc);
    let display = format!("{}", err);
    assert!(
        display.contains("403"),
        "display should contain status code"
    );
    assert!(
        display.contains("response"),
        "display should contain 'response'"
    );
}

#[test]
fn error_source_response_error_is_none() {
    use std::error::Error as StdError;
    let rc = img_src::apis::ResponseContent::<String> {
        status: reqwest::StatusCode::NOT_FOUND,
        content: "not found".into(),
        entity: None,
    };
    let err: img_src::apis::Error<String> = img_src::apis::Error::ResponseError(rc);
    assert!(
        err.source().is_none(),
        "ResponseError should have no source"
    );
}

#[test]
fn error_source_serde_is_some() {
    use std::error::Error as StdError;
    let serde_err = serde_json::from_str::<String>("{{bad}}").unwrap_err();
    let err: img_src::apis::Error<String> = img_src::apis::Error::Serde(serde_err);
    assert!(err.source().is_some(), "Serde error should have a source");
}
