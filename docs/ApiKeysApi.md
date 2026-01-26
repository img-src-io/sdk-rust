# \ApiKeysApi

All URIs are relative to *https://api.img-src.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_api_key**](ApiKeysApi.md#create_api_key) | **POST** /api/v1/settings/api-keys | Create API key
[**delete_api_key**](ApiKeysApi.md#delete_api_key) | **DELETE** /api/v1/settings/api-keys/{id} | Delete API key
[**list_api_keys**](ApiKeysApi.md#list_api_keys) | **GET** /api/v1/settings/api-keys | List API keys



## create_api_key

> models::CreateApiKeyResponse create_api_key(create_api_key_request)
Create API key

Creates a new API key for the authenticated user. The full key is only returned once.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_api_key_request** | Option<[**CreateApiKeyRequest**](CreateApiKeyRequest.md)> |  |  |

### Return type

[**models::CreateApiKeyResponse**](CreateApiKeyResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_key

> models::DeleteApiKeyResponse delete_api_key(id)
Delete API key

Deletes an API key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::DeleteApiKeyResponse**](DeleteApiKeyResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_api_keys

> models::ApiKeyListResponse list_api_keys()
List API keys

Returns all API keys for the authenticated user

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiKeyListResponse**](ApiKeyListResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

