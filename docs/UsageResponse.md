# UsageResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**plan** | **String** | User's plan ID | 
**plan_name** | **String** | Human-readable plan name | 
**plan_status** | **PlanStatus** | Current plan status (enum: active, cancelling, expired) | 
**subscription_ends_at** | **i32** | Unix timestamp when subscription ends (for cancelling plans) | 
**plan_limits** | [**models::PlanLimits**](PlanLimits.md) |  | 
**total_images** | **i32** | Total images (lifetime) | 
**storage_used_bytes** | **i32** | Total storage used in bytes | 
**storage_used_mb** | **f64** | Total storage used in MB | 
**storage_used_gb** | **f64** | Total storage used in GB | 
**current_period** | [**models::CurrentPeriod**](CurrentPeriod.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


