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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostV1EventsEventIdAttendanceRequest {
    /// ID of the user for whom you are RSVP'ing. Must be within your scope as a user AND must be invited to the event.
    #[serde(rename = "event_trackers[user_id]")]
    pub event_trackers_left_square_bracket_user_id_right_square_bracket: Vec<i32>,
    /// 0: Did not attend. 1: Attended. 2: No clue.
    #[serde(rename = "event_trackers[attendance_status_cd]")]
    pub event_trackers_left_square_bracket_attendance_status_cd_right_square_bracket: Vec<String>,
    /// How many adult guests will this person bring?
    #[serde(rename = "event_trackers[number_of_adult_guests]", skip_serializing_if = "Option::is_none")]
    pub event_trackers_left_square_bracket_number_of_adult_guests_right_square_bracket: Option<Vec<i32>>,
    /// How many youth guests will this person bring?
    #[serde(rename = "event_trackers[number_of_youth_guests]", skip_serializing_if = "Option::is_none")]
    pub event_trackers_left_square_bracket_number_of_youth_guests_right_square_bracket: Option<Vec<i32>>,
}

impl PostV1EventsEventIdAttendanceRequest {
    pub fn new(event_trackers_left_square_bracket_user_id_right_square_bracket: Vec<i32>, event_trackers_left_square_bracket_attendance_status_cd_right_square_bracket: Vec<String>) -> PostV1EventsEventIdAttendanceRequest {
        PostV1EventsEventIdAttendanceRequest {
            event_trackers_left_square_bracket_user_id_right_square_bracket,
            event_trackers_left_square_bracket_attendance_status_cd_right_square_bracket,
            event_trackers_left_square_bracket_number_of_adult_guests_right_square_bracket: None,
            event_trackers_left_square_bracket_number_of_youth_guests_right_square_bracket: None,
        }
    }
}

