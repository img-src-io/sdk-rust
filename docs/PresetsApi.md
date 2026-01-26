# \PresetsApi

All URIs are relative to *https://api.img-src.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_preset**](PresetsApi.md#create_preset) | **POST** /api/v1/settings/presets | Create preset
[**delete_preset**](PresetsApi.md#delete_preset) | **DELETE** /api/v1/settings/presets/{id} | Delete preset
[**get_preset**](PresetsApi.md#get_preset) | **GET** /api/v1/settings/presets/{id} | Get preset
[**list_presets**](PresetsApi.md#list_presets) | **GET** /api/v1/settings/presets | List presets
[**update_preset**](PresetsApi.md#update_preset) | **PUT** /api/v1/settings/presets/{id} | Update preset



## create_preset

> models::Preset create_preset(create_preset_request)
Create preset

Creates a new transformation preset. Requires Pro plan.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_preset_request** | Option<[**CreatePresetRequest**](CreatePresetRequest.md)> |  |  |

### Return type

[**models::Preset**](Preset.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_preset

> models::DeletePresetResponse delete_preset(id)
Delete preset

Deletes a preset. Requires Pro plan.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::DeletePresetResponse**](DeletePresetResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_preset

> models::Preset get_preset(id)
Get preset

Returns a specific preset by ID. Requires Pro plan.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::Preset**](Preset.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_presets

> models::ListPresetsResponse list_presets()
List presets

Returns all transformation presets for the authenticated user. Requires Pro plan.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListPresetsResponse**](ListPresetsResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_preset

> models::Preset update_preset(id, update_preset_request)
Update preset

Updates an existing preset. Requires Pro plan.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**update_preset_request** | Option<[**UpdatePresetRequest**](UpdatePresetRequest.md)> |  |  |

### Return type

[**models::Preset**](Preset.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

