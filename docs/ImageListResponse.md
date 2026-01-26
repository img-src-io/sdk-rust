# ImageListResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**images** | [**Vec<models::ImageListItem>**](ImageListItem.md) |  | 
**folders** | [**Vec<models::FolderItem>**](FolderItem.md) |  | 
**total** | **i32** | Total count of images (in current path) | 
**limit** | **i32** | Maximum items per page | 
**offset** | **i32** | Current offset | 
**has_more** | **bool** | Whether more items exist | 
**path_filter** | Option<**String**> | Current path filter (if any) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


