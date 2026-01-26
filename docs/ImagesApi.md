# \ImagesApi

All URIs are relative to *https://api.img-src.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_signed_url**](ImagesApi.md#create_signed_url) | **POST** /api/v1/images/{id}/signed-url | Create signed URL
[**delete_image**](ImagesApi.md#delete_image) | **DELETE** /api/v1/images/{id} | Delete image
[**delete_image_path**](ImagesApi.md#delete_image_path) | **DELETE** /api/v1/images/path/{username}/{filepath} | Delete image path
[**get_image**](ImagesApi.md#get_image) | **GET** /api/v1/images/{id} | Get image metadata
[**list_images**](ImagesApi.md#list_images) | **GET** /api/v1/images | List images
[**search_images**](ImagesApi.md#search_images) | **GET** /api/v1/images/search | Search images
[**upload_image**](ImagesApi.md#upload_image) | **POST** /api/v1/images | Upload image



## create_signed_url

> models::SignedUrlResponse create_signed_url(id, create_signed_url_request)
Create signed URL

Create a time-limited signed URL for an image (Pro plan only)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**create_signed_url_request** | Option<[**CreateSignedUrlRequest**](CreateSignedUrlRequest.md)> |  |  |

### Return type

[**models::SignedUrlResponse**](SignedUrlResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_image

> models::DeleteResponse delete_image(id)
Delete image

Delete an image and all its paths

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::DeleteResponse**](DeleteResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_image_path

> models::PathDeleteResponse delete_image_path(username, filepath)
Delete image path

Delete a specific path from an image. If this is the last path, the image is deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**filepath** | **String** |  | [required] |

### Return type

[**models::PathDeleteResponse**](PathDeleteResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_image

> models::MetadataResponse get_image(id)
Get image metadata

Get metadata for a specific image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::MetadataResponse**](MetadataResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_images

> models::ImageListResponse list_images(limit, offset, path)
List images

List user's images with pagination and optional path filtering

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |[default to 50]
**offset** | Option<**i32**> |  |  |[default to 0]
**path** | Option<**String**> |  |  |

### Return type

[**models::ImageListResponse**](ImageListResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_images

> models::SearchResponse search_images(q, limit)
Search images

Search images by filename

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |[default to 20]

### Return type

[**models::SearchResponse**](SearchResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_image

> models::UploadResponse upload_image(file, target_path)
Upload image

Upload a new image. Supports multipart/form-data with 'file' field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | Option<**std::path::PathBuf**> | Image file to upload |  |
**target_path** | Option<**String**> | Target path for organizing the image |  |

### Return type

[**models::UploadResponse**](UploadResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

