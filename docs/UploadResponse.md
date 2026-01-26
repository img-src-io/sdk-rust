# UploadResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Image ID (first 16 characters of SHA256 hash) | 
**hash** | **String** | Full SHA256 hash of the image content | 
**url** | **String** | Primary CDN URL for the image | 
**paths** | **Vec<String>** | All paths where this image is accessible | 
**is_new** | Option<**bool**> | Whether this is a newly uploaded image (false if duplicate) | [optional]
**size** | **i32** | File size in bytes | 
**format** | **String** | Detected image format | 
**dimensions** | Option<[**models::ImageDimensions**](ImageDimensions.md)> |  | [optional]
**available_formats** | [**models::AvailableFormats**](AvailableFormats.md) |  | 
**uploaded_at** | **String** | Upload timestamp (RFC3339 format) | 
**_links** | [**models::HateoasLinks**](HateoasLinks.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


