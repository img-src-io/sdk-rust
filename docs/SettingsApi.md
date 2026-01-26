# \SettingsApi

All URIs are relative to *https://api.img-src.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_settings**](SettingsApi.md#get_settings) | **GET** /api/v1/settings | Get user settings
[**update_settings**](SettingsApi.md#update_settings) | **PUT** /api/v1/settings | Update user settings



## get_settings

> models::SettingsResponse get_settings()
Get user settings

Returns the authenticated user's settings

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SettingsResponse**](SettingsResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_settings

> models::SettingsUpdateResponse update_settings(update_settings_request)
Update user settings

Updates the authenticated user's settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_settings_request** | Option<[**UpdateSettingsRequest**](UpdateSettingsRequest.md)> |  |  |

### Return type

[**models::SettingsUpdateResponse**](SettingsUpdateResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

