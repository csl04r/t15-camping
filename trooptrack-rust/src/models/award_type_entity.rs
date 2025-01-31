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

/// AwardTypeEntity : AwardTypeEntity model
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwardTypeEntity {
    /// ID of the Award Type Record
    #[serde(rename = "award_type_id", skip_serializing_if = "Option::is_none")]
    pub award_type_id: Option<i32>,
    /// Name of the Award Type
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Array of Achievements for this award type
    #[serde(rename = "active_achievements", skip_serializing_if = "Option::is_none")]
    pub active_achievements: Option<Vec<models::AchievementEntity>>,
}

impl AwardTypeEntity {
    /// AwardTypeEntity model
    pub fn new() -> AwardTypeEntity {
        AwardTypeEntity {
            award_type_id: None,
            name: None,
            active_achievements: None,
        }
    }
}

