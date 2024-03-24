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

/// UserPrivilegesEntity : UserPrivilegesEntity model
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserPrivilegesEntity {
    /// API authentication token for this user
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// ID of the user record
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    /// Full number or name identifying the unit this user belongs to.
    #[serde(rename = "troop", skip_serializing_if = "Option::is_none")]
    pub troop: Option<String>,
    /// ID of the unit this user belongs to
    #[serde(rename = "troop_id", skip_serializing_if = "Option::is_none")]
    pub troop_id: Option<i32>,
    /// This value is used to identify the contents of the offline cache that apply to this user.
    #[serde(rename = "cache_scope", skip_serializing_if = "Option::is_none")]
    pub cache_scope: Option<String>,
    /// Number of the unit this user belongs to.
    #[serde(rename = "troop_number", skip_serializing_if = "Option::is_none")]
    pub troop_number: Option<String>,
    /// Description of the type of unit this user belongs to.
    #[serde(rename = "troop_type", skip_serializing_if = "Option::is_none")]
    pub troop_type: Option<String>,
    /// ID of the unit type record
    #[serde(rename = "troop_type_id", skip_serializing_if = "Option::is_none")]
    pub troop_type_id: Option<i32>,
    /// Names of privileges this user has.
    #[serde(rename = "privileges", skip_serializing_if = "Option::is_none")]
    pub privileges: Option<Vec<String>>,
}

impl UserPrivilegesEntity {
    /// UserPrivilegesEntity model
    pub fn new() -> UserPrivilegesEntity {
        UserPrivilegesEntity {
            token: None,
            user_id: None,
            troop: None,
            troop_id: None,
            cache_scope: None,
            troop_number: None,
            troop_type: None,
            troop_type_id: None,
            privileges: None,
        }
    }
}

