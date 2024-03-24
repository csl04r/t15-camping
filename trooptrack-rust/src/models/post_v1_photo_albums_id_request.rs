/*
 * TroopTrack API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostV1PhotoAlbumsIdRequest {
    #[serde(rename = "photo_file_extension")]
    pub photo_file_extension: String,
    #[serde(rename = "photo_file_content")]
    pub photo_file_content: String,
    #[serde(rename = "album_name", skip_serializing_if = "Option::is_none")]
    pub album_name: Option<String>,
}

impl PostV1PhotoAlbumsIdRequest {
    pub fn new(photo_file_extension: String, photo_file_content: String) -> PostV1PhotoAlbumsIdRequest {
        PostV1PhotoAlbumsIdRequest {
            photo_file_extension,
            photo_file_content,
            album_name: None,
        }
    }
}
