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
pub struct PostV1UsersIdRequest {
    #[serde(rename = "user[last_name]", skip_serializing_if = "Option::is_none")]
    pub user_left_square_bracket_last_name_right_square_bracket: Option<String>,
    #[serde(rename = "user[first_name]", skip_serializing_if = "Option::is_none")]
    pub user_left_square_bracket_first_name_right_square_bracket: Option<String>,
    #[serde(rename = "user[email]", skip_serializing_if = "Option::is_none")]
    pub user_left_square_bracket_email_right_square_bracket: Option<String>,
    #[serde(rename = "user[born_on]", skip_serializing_if = "Option::is_none")]
    pub user_left_square_bracket_born_on_right_square_bracket: Option<String>,
    #[serde(rename = "user[home_phone]", skip_serializing_if = "Option::is_none")]
    pub user_left_square_bracket_home_phone_right_square_bracket: Option<String>,
    #[serde(rename = "user[cell_phone]", skip_serializing_if = "Option::is_none")]
    pub user_left_square_bracket_cell_phone_right_square_bracket: Option<String>,
    #[serde(rename = "user[profile_photo_file_name]", skip_serializing_if = "Option::is_none")]
    pub user_left_square_bracket_profile_photo_file_name_right_square_bracket: Option<String>,
    #[serde(rename = "user[profile_photo_file_content]", skip_serializing_if = "Option::is_none")]
    pub user_left_square_bracket_profile_photo_file_content_right_square_bracket: Option<String>,
}

impl PostV1UsersIdRequest {
    pub fn new() -> PostV1UsersIdRequest {
        PostV1UsersIdRequest {
            user_left_square_bracket_last_name_right_square_bracket: None,
            user_left_square_bracket_first_name_right_square_bracket: None,
            user_left_square_bracket_email_right_square_bracket: None,
            user_left_square_bracket_born_on_right_square_bracket: None,
            user_left_square_bracket_home_phone_right_square_bracket: None,
            user_left_square_bracket_cell_phone_right_square_bracket: None,
            user_left_square_bracket_profile_photo_file_name_right_square_bracket: None,
            user_left_square_bracket_profile_photo_file_content_right_square_bracket: None,
        }
    }
}

