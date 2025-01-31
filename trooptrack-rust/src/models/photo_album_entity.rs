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

/// PhotoAlbumEntity : PhotoAlbumEntity model
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhotoAlbumEntity {
    /// Name used to identify the photo album
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// ID of the photo album record
    #[serde(rename = "photo_album_id", skip_serializing_if = "Option::is_none")]
    pub photo_album_id: Option<i32>,
    /// Number of photos in this album
    #[serde(rename = "photo_count", skip_serializing_if = "Option::is_none")]
    pub photo_count: Option<i32>,
    /// Date the album was taken on as specified by the user
    #[serde(rename = "taken_on", skip_serializing_if = "Option::is_none")]
    pub taken_on: Option<String>,
    /// An array of troop photos
    #[serde(rename = "troop_photos", skip_serializing_if = "Option::is_none")]
    pub troop_photos: Option<Vec<models::TroopPhotoEntity>>,
}

impl PhotoAlbumEntity {
    /// PhotoAlbumEntity model
    pub fn new() -> PhotoAlbumEntity {
        PhotoAlbumEntity {
            name: None,
            photo_album_id: None,
            photo_count: None,
            taken_on: None,
            troop_photos: None,
        }
    }
}

