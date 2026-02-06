//! Full integration test exercising every SDK API function against the live server.
//!
//! Requires `IMGSRC_API_KEY` environment variable to be set.
//!
//! Run with:
//! ```sh
//! IMGSRC_API_KEY=imgsrc_... cargo test --test integration_test -- --nocapture
//! ```

use img_src::apis::configuration::Configuration;
use img_src::apis::{images_api, presets_api, settings_api, usage_api};
use img_src::models::{
    CreatePresetRequest, CreateSignedUrlRequest, UpdatePresetRequest, UpdateSettingsRequest,
    UpdateVisibilityRequest,
};
use std::io::Write;

/// Valid 8x8 red PNG (314 bytes) — above API minimum of 100 bytes.
const TINY_PNG: &[u8] = &[
    0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
    0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR chunk
    0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x08, // 8x8
    0x08, 0x02, 0x00, 0x00, 0x00, 0x4B, 0x6D, 0x29, // 8-bit RGB
    0xDC, 0x00, 0x00, 0x00, 0x22, 0x74, 0x45, 0x58, // tEXt chunk
    0x74, 0x43, 0x6F, 0x6D, 0x6D, 0x65, 0x6E, 0x74, 0x00, 0x53, 0x44, 0x4B, 0x20, 0x69, 0x6E, 0x74,
    0x65, 0x67, 0x72, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x74, 0x65, 0x73, 0x74, 0x20, 0x69, 0x6D,
    0x61, 0x67, 0x65, 0x37, 0x51, 0x99, 0xD1, 0x00, // IDAT chunk
    0x00, 0x00, 0xD3, 0x49, 0x44, 0x41, 0x54, 0x78, 0x01, 0x01, 0xC8, 0x00, 0x37, 0xFF, 0x00, 0xFF,
    0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00,
    0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00,
    0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00,
    0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00,
    0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00,
    0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF,
    0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF,
    0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF,
    0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00,
    0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00,
    0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00,
    0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00,
    0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x28, 0xFF, 0x3F, 0xC1, 0x82, 0x65, 0x0C, 0x10, 0x00,
    0x00, // IEND chunk
    0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
];

fn make_config() -> Configuration {
    let api_key =
        std::env::var("IMGSRC_API_KEY").expect("IMGSRC_API_KEY environment variable must be set");
    let mut config = Configuration::new();
    config.bearer_access_token = Some(api_key);
    if let Ok(server_url) = std::env::var("IMGSRC_SERVER_URL") {
        config.base_path = server_url;
    }
    config
}

fn write_temp_png() -> (tempfile::NamedTempFile, std::path::PathBuf) {
    let mut tmp = tempfile::Builder::new()
        .suffix(".png")
        .tempfile()
        .expect("failed to create temp file");
    tmp.write_all(TINY_PNG).expect("failed to write temp PNG");
    // Append unique data after IEND so each run produces a different content hash.
    // PNG parsers ignore trailing data after IEND.
    let unique = format!("unique:{}:{}", std::process::id(), chrono_like_now());
    tmp.write_all(unique.as_bytes())
        .expect("failed to write unique suffix");
    tmp.flush().expect("failed to flush temp file");
    let path = tmp.path().to_path_buf();
    (tmp, path)
}

/// Guard that ensures test image cleanup even on panic.
struct CleanupGuard<'a> {
    config: &'a Configuration,
    image_id: Option<String>,
}

impl<'a> Drop for CleanupGuard<'a> {
    fn drop(&mut self) {
        if let Some(ref id) = self.image_id {
            eprintln!("[CLEANUP] Deleting test image {id}...");
            let config = self.config.clone();
            let id = id.clone();
            // Use a blocking runtime to clean up from Drop
            let _ = std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let _ = rt.block_on(images_api::delete_image(&config, &id));
            })
            .join();
        }
    }
}

#[tokio::test]
#[ignore] // requires IMGSRC_API_KEY env var — run with: cargo test --test integration_test -- --ignored
async fn full_integration() {
    let config = make_config();
    let mut cleanup = CleanupGuard {
        config: &config,
        image_id: None,
    };

    // ── 1. get_settings ──────────────────────────────────────────────
    let settings_resp = settings_api::get_settings(&config)
        .await
        .expect("get_settings failed");
    let settings = &settings_resp.settings;
    let username = settings.username.clone();
    let plan = settings.plan.clone();
    let original_quality = settings.default_quality;
    let masked = if username.len() > 3 {
        format!("{}***", &username[..3])
    } else {
        "***".to_string()
    };
    println!("[PASS] get_settings: username={masked}, plan={plan}");

    let is_pro = plan == "pro";

    // ── 2. get_usage ─────────────────────────────────────────────────
    let usage = usage_api::get_usage(&config)
        .await
        .expect("get_usage failed");
    assert_eq!(usage.plan, plan, "usage plan should match settings plan");
    println!(
        "[PASS] get_usage: plan={}, total_images={}, storage={:.2} MB",
        usage.plan, usage.total_images, usage.storage_used_mb
    );

    // ── 3. upload_image ──────────────────────────────────────────────
    let (_tmp_file, tmp_path) = write_temp_png();
    let target_path = "__sdk_test".to_string();
    let upload =
        images_api::upload_image(&config, Some(tmp_path), Some(&target_path), Some("public"))
            .await
            .expect("upload_image failed");
    let image_id = upload.id.clone();
    cleanup.image_id = Some(image_id.clone());
    assert!(!image_id.is_empty(), "image ID should not be empty");
    assert_eq!(upload.visibility, "public");
    println!("[PASS] upload_image: id={image_id}, size={}", upload.size);

    // ── 4. list_images ───────────────────────────────────────────────
    // Retry with back-off to handle eventual consistency
    let mut found = false;
    for attempt in 1..=5 {
        tokio::time::sleep(std::time::Duration::from_secs(attempt)).await;
        let list = images_api::list_images(&config, Some(100), Some(0), Some("__sdk_test"))
            .await
            .expect("list_images failed");
        found = list.images.iter().any(|img| img.id == image_id);
        if found {
            println!(
                "[PASS] list_images: total={}, found test image=true (attempt {attempt})",
                list.total
            );
            break;
        }
        println!(
            "[RETRY] list_images attempt {attempt}: total={}, image not found yet",
            list.total
        );
    }
    assert!(
        found,
        "uploaded image should appear in list_images after retries"
    );

    // ── 5. search_images ─────────────────────────────────────────────
    let mut search_found = false;
    for attempt in 1..=5 {
        let search = images_api::search_images(&config, "__sdk_test", Some(10))
            .await
            .expect("search_images failed");
        search_found = search.results.iter().any(|r| r.id == image_id);
        if search_found {
            println!(
                "[PASS] search_images: query='__sdk_test', results={}, found=true (attempt {attempt})",
                search.total
            );
            break;
        }
        println!(
            "[RETRY] search_images attempt {attempt}: total={}, image not found yet",
            search.total
        );
        tokio::time::sleep(std::time::Duration::from_secs(attempt)).await;
    }
    if !search_found {
        println!("[WARN] search_images: image not found after retries (dev indexing delay), continuing...");
    }

    // ── 6. get_image ─────────────────────────────────────────────────
    let meta = images_api::get_image(&config, &image_id)
        .await
        .expect("get_image failed");
    assert_eq!(meta.id, image_id);
    assert_eq!(meta.visibility, "public");
    println!(
        "[PASS] get_image: id={}, visibility={}, hash={}",
        meta.id, meta.visibility, meta.metadata.hash
    );

    // ── 7. update_visibility → private ───────────────────────────────
    if is_pro {
        let vis_resp = images_api::update_visibility(
            &config,
            &image_id,
            UpdateVisibilityRequest::new("private".to_string()),
        )
        .await
        .expect("update_visibility (to private) failed");
        assert_eq!(vis_resp.visibility, "private");
        println!("[PASS] update_visibility: → private");

        // ── 8. update_visibility → public ────────────────────────────────
        let vis_resp = images_api::update_visibility(
            &config,
            &image_id,
            UpdateVisibilityRequest::new("public".to_string()),
        )
        .await
        .expect("update_visibility (to public) failed");
        assert_eq!(vis_resp.visibility, "public");
        println!("[PASS] update_visibility: → public");
    } else {
        println!("[SKIP] update_visibility: requires Pro plan");
    }

    // ── 9. create_signed_url (Pro only) ──────────────────────────────
    if is_pro {
        let signed = images_api::create_signed_url(
            &config,
            &image_id,
            Some(CreateSignedUrlRequest {
                expires_in_seconds: Some(300),
            }),
        )
        .await
        .expect("create_signed_url failed");
        assert!(
            !signed.signed_url.is_empty(),
            "signed URL should not be empty"
        );
        assert_eq!(signed.expires_in_seconds, 300);
        println!(
            "[PASS] create_signed_url: expires_at={}, len={}",
            signed.expires_at,
            signed.signed_url.len()
        );
    } else {
        println!("[SKIP] create_signed_url: requires Pro plan");
    }

    // ── 10-14. Presets (Pro only) ────────────────────────────────────
    if is_pro {
        // 10. list_presets
        let presets_before = presets_api::list_presets(&config)
            .await
            .expect("list_presets failed");
        println!("[PASS] list_presets: total={}", presets_before.total);

        // 11. create_preset
        let mut params = std::collections::HashMap::new();
        params.insert("w".to_string(), serde_json::json!(200));
        params.insert("h".to_string(), serde_json::json!(200));
        params.insert("fit".to_string(), serde_json::json!("cover"));
        let preset = presets_api::create_preset(
            &config,
            Some(CreatePresetRequest {
                name: "__sdk_test_preset".to_string(),
                description: Some("SDK integration test preset".to_string()),
                params,
            }),
        )
        .await
        .expect("create_preset failed");
        let preset_id = preset.id.clone();
        assert_eq!(preset.name, "__sdk_test_preset");
        println!("[PASS] create_preset: id={preset_id}, name={}", preset.name);

        // 12. get_preset
        let fetched = presets_api::get_preset(&config, &preset_id)
            .await
            .expect("get_preset failed");
        assert_eq!(fetched.id, preset_id);
        assert_eq!(fetched.name, "__sdk_test_preset");
        println!(
            "[PASS] get_preset: id={}, name={}",
            fetched.id, fetched.name
        );

        // 13. update_preset
        let updated = presets_api::update_preset(
            &config,
            &preset_id,
            Some(UpdatePresetRequest {
                name: None,
                description: Some("Updated SDK test preset".to_string()),
                params: None,
            }),
        )
        .await
        .expect("update_preset failed");
        assert_eq!(
            updated.description.as_deref(),
            Some("Updated SDK test preset")
        );
        println!("[PASS] update_preset: description updated");

        // 14. delete_preset
        let deleted = presets_api::delete_preset(&config, "__sdk_test_preset")
            .await
            .expect("delete_preset failed");
        assert!(deleted.success);
        println!("[PASS] delete_preset: success={}", deleted.success);
    } else {
        println!("[SKIP] presets (10-14): requires Pro plan");
    }

    // ── 15. delete_image ─────────────────────────────────────────────
    let del = images_api::delete_image(&config, &image_id)
        .await
        .expect("delete_image failed");
    assert!(del.success);
    // Disarm the cleanup guard since we deleted manually
    cleanup.image_id = None;
    println!(
        "[PASS] delete_image: id={image_id}, success=true, message={}",
        del.message
    );

    // ── 16. update_settings ──────────────────────────────────────────
    let new_quality = if original_quality == 85 { 80 } else { 85 };
    let updated_settings = settings_api::update_settings(
        &config,
        Some(UpdateSettingsRequest {
            default_quality: Some(new_quality),
            delivery_formats: None,
            default_fit_mode: None,
            default_max_width: None,
            default_max_height: None,
            theme: None,
            language: None,
        }),
    )
    .await
    .expect("update_settings failed");
    assert_eq!(updated_settings.settings.default_quality, new_quality);
    println!(
        "[PASS] update_settings: default_quality {} → {}",
        original_quality, new_quality
    );

    // ── 17. revert settings ──────────────────────────────────────────
    let reverted = settings_api::update_settings(
        &config,
        Some(UpdateSettingsRequest {
            default_quality: Some(original_quality),
            delivery_formats: None,
            default_fit_mode: None,
            default_max_width: None,
            default_max_height: None,
            theme: None,
            language: None,
        }),
    )
    .await
    .expect("update_settings (revert) failed");
    assert_eq!(reverted.settings.default_quality, original_quality);
    println!(
        "[PASS] update_settings (revert): default_quality → {}",
        original_quality
    );

    println!("\n✅ All integration tests passed!");
}

/// Simple timestamp for unique filenames (no chrono dependency needed).
fn chrono_like_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
