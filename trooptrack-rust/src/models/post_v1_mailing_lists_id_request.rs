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
pub struct PostV1MailingListsIdRequest {
    #[serde(rename = "message[subject]")]
    pub message_left_square_bracket_subject_right_square_bracket: String,
    #[serde(rename = "message[body]")]
    pub message_left_square_bracket_body_right_square_bracket: String,
}

impl PostV1MailingListsIdRequest {
    pub fn new(message_left_square_bracket_subject_right_square_bracket: String, message_left_square_bracket_body_right_square_bracket: String) -> PostV1MailingListsIdRequest {
        PostV1MailingListsIdRequest {
            message_left_square_bracket_subject_right_square_bracket,
            message_left_square_bracket_body_right_square_bracket,
        }
    }
}

