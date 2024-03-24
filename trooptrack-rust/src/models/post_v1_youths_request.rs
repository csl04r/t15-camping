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
pub struct PostV1YouthsRequest {
    #[serde(rename = "youth[last_name]")]
    pub youth_left_square_bracket_last_name_right_square_bracket: String,
    #[serde(rename = "youth[first_name]")]
    pub youth_left_square_bracket_first_name_right_square_bracket: String,
    #[serde(rename = "youth[email]", skip_serializing_if = "Option::is_none")]
    pub youth_left_square_bracket_email_right_square_bracket: Option<String>,
    #[serde(rename = "youth[born_on]", skip_serializing_if = "Option::is_none")]
    pub youth_left_square_bracket_born_on_right_square_bracket: Option<String>,
    #[serde(rename = "youth[scout]", skip_serializing_if = "Option::is_none")]
    pub youth_left_square_bracket_scout_right_square_bracket: Option<String>,
    #[serde(rename = "youth[home_phone]", skip_serializing_if = "Option::is_none")]
    pub youth_left_square_bracket_home_phone_right_square_bracket: Option<String>,
    #[serde(rename = "youth[cell_phone]", skip_serializing_if = "Option::is_none")]
    pub youth_left_square_bracket_cell_phone_right_square_bracket: Option<String>,
    #[serde(rename = "youth[patrol_id]", skip_serializing_if = "Option::is_none")]
    pub youth_left_square_bracket_patrol_id_right_square_bracket: Option<i32>,
    #[serde(rename = "youth[household_id]", skip_serializing_if = "Option::is_none")]
    pub youth_left_square_bracket_household_id_right_square_bracket: Option<i32>,
    #[serde(rename = "youth[profile_photo]", skip_serializing_if = "Option::is_none")]
    pub youth_left_square_bracket_profile_photo_right_square_bracket: Option<String>,
    #[serde(rename = "youth[leadership_position_id]", skip_serializing_if = "Option::is_none")]
    pub youth_left_square_bracket_leadership_position_id_right_square_bracket: Option<i32>,
    #[serde(rename = "youth[create_money_account]", skip_serializing_if = "Option::is_none")]
    pub youth_left_square_bracket_create_money_account_right_square_bracket: Option<bool>,
    #[serde(rename = "youth[send_invite]", skip_serializing_if = "Option::is_none")]
    pub youth_left_square_bracket_send_invite_right_square_bracket: Option<bool>,
}

impl PostV1YouthsRequest {
    pub fn new(youth_left_square_bracket_last_name_right_square_bracket: String, youth_left_square_bracket_first_name_right_square_bracket: String) -> PostV1YouthsRequest {
        PostV1YouthsRequest {
            youth_left_square_bracket_last_name_right_square_bracket,
            youth_left_square_bracket_first_name_right_square_bracket,
            youth_left_square_bracket_email_right_square_bracket: None,
            youth_left_square_bracket_born_on_right_square_bracket: None,
            youth_left_square_bracket_scout_right_square_bracket: None,
            youth_left_square_bracket_home_phone_right_square_bracket: None,
            youth_left_square_bracket_cell_phone_right_square_bracket: None,
            youth_left_square_bracket_patrol_id_right_square_bracket: None,
            youth_left_square_bracket_household_id_right_square_bracket: None,
            youth_left_square_bracket_profile_photo_right_square_bracket: None,
            youth_left_square_bracket_leadership_position_id_right_square_bracket: None,
            youth_left_square_bracket_create_money_account_right_square_bracket: None,
            youth_left_square_bracket_send_invite_right_square_bracket: None,
        }
    }
}
