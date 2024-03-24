/*
 * API title
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// AchievementEntity : AchievementEntity model
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AchievementEntity {
    /// Name of the achievement
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Detailed description of the achievement
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Array of requirements
    #[serde(rename = "children", skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<models::AchievementEntity>>,
}

impl AchievementEntity {
    /// AchievementEntity model
    pub fn new() -> AchievementEntity {
        AchievementEntity {
            name: None,
            description: None,
            children: None,
        }
    }
}

