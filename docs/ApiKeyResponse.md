# ApiKeyResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | API key UUID | 
**name** | **String** | Key name | 
**key_prefix** | **String** | Key prefix (first 14 chars) | 
**scopes** | **String** | Granted scopes | 
**created_at** | **i32** | Creation timestamp (Unix epoch) | 
**last_used_at** | Option<**i32**> | Last used timestamp (Unix epoch) | [optional]
**expires_at** | Option<**i32**> | Expiration timestamp (Unix epoch) | [optional]
**total_requests** | **i32** | Total requests made with this key | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


