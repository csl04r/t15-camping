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
pub struct PostV1EventsRequest {
    #[serde(rename = "event[title]")]
    pub event_left_square_bracket_title_right_square_bracket: String,
    /// ID of an event type returned by the events/available_types API
    #[serde(rename = "event[event_type_id]")]
    pub event_left_square_bracket_event_type_id_right_square_bracket: String,
    /// Format: YYYY-MM-DDTHH:MM-0600 i.e. 2015-07-04T13:00-0600
    #[serde(rename = "event[start_at]")]
    pub event_left_square_bracket_start_at_right_square_bracket: String,
    /// Format: YYYY-MM-DDTHH:MM-0600 i.e. 2015-07-04T13:00-0600
    #[serde(rename = "event[end_at]", skip_serializing_if = "Option::is_none")]
    pub event_left_square_bracket_end_at_right_square_bracket: Option<String>,
    #[serde(rename = "event[location]", skip_serializing_if = "Option::is_none")]
    pub event_left_square_bracket_location_right_square_bracket: Option<String>,
    #[serde(rename = "event[description]")]
    pub event_left_square_bracket_description_right_square_bracket: String,
    /// An array of strings describing who to invite. Each token must include the class and ID of the Troop, Patrol, or User to invite.               For example, to invite the whole troop, you would provide ['Troop-1211'], or to invite patrol 11911 and user 1223 and user 3344 then you would               provide ['Patrol-11911', 'User-1223', 'User-3344']
    #[serde(rename = "event[inviteable_tokens]", skip_serializing_if = "Option::is_none")]
    pub event_left_square_bracket_inviteable_tokens_right_square_bracket: Option<String>,
    /// How many nights do you expect to camp?
    #[serde(rename = "event[camping_nights]", skip_serializing_if = "Option::is_none")]
    pub event_left_square_bracket_camping_nights_right_square_bracket: Option<String>,
    /// Expected number of miles to be hiked
    #[serde(rename = "event[hiking_miles]", skip_serializing_if = "Option::is_none")]
    pub event_left_square_bracket_hiking_miles_right_square_bracket: Option<String>,
    /// Expected number of miles to be traveled by canoe
    #[serde(rename = "event[canoeing_miles]", skip_serializing_if = "Option::is_none")]
    pub event_left_square_bracket_canoeing_miles_right_square_bracket: Option<String>,
    /// Fee for youth to attend
    #[serde(rename = "event[dues]", skip_serializing_if = "Option::is_none")]
    pub event_left_square_bracket_dues_right_square_bracket: Option<String>,
    /// Fee for adults to attend
    #[serde(rename = "event[adult_fee]", skip_serializing_if = "Option::is_none")]
    pub event_left_square_bracket_adult_fee_right_square_bracket: Option<String>,
    #[serde(rename = "event[rsvp_deadline]", skip_serializing_if = "Option::is_none")]
    pub event_left_square_bracket_rsvp_deadline_right_square_bracket: Option<String>,
    /// Number of days before the event to send an invitation. Use 999 to indicate sending the invite immediately.
    #[serde(rename = "event[send_invites_when]", skip_serializing_if = "Option::is_none")]
    pub event_left_square_bracket_send_invites_when_right_square_bracket: Option<String>,
    /// Number of days before the event to send a reminder.
    #[serde(rename = "event[send_reminder_when]", skip_serializing_if = "Option::is_none")]
    pub event_left_square_bracket_send_reminder_when_right_square_bracket: Option<String>,
    /// Number of service hours expected to be completed
    #[serde(rename = "event[service_hours]", skip_serializing_if = "Option::is_none")]
    pub event_left_square_bracket_service_hours_right_square_bracket: Option<String>,
}

impl PostV1EventsRequest {
    pub fn new(event_left_square_bracket_title_right_square_bracket: String, event_left_square_bracket_event_type_id_right_square_bracket: String, event_left_square_bracket_start_at_right_square_bracket: String, event_left_square_bracket_description_right_square_bracket: String) -> PostV1EventsRequest {
        PostV1EventsRequest {
            event_left_square_bracket_title_right_square_bracket,
            event_left_square_bracket_event_type_id_right_square_bracket,
            event_left_square_bracket_start_at_right_square_bracket,
            event_left_square_bracket_end_at_right_square_bracket: None,
            event_left_square_bracket_location_right_square_bracket: None,
            event_left_square_bracket_description_right_square_bracket,
            event_left_square_bracket_inviteable_tokens_right_square_bracket: None,
            event_left_square_bracket_camping_nights_right_square_bracket: None,
            event_left_square_bracket_hiking_miles_right_square_bracket: None,
            event_left_square_bracket_canoeing_miles_right_square_bracket: None,
            event_left_square_bracket_dues_right_square_bracket: None,
            event_left_square_bracket_adult_fee_right_square_bracket: None,
            event_left_square_bracket_rsvp_deadline_right_square_bracket: None,
            event_left_square_bracket_send_invites_when_right_square_bracket: None,
            event_left_square_bracket_send_reminder_when_right_square_bracket: None,
            event_left_square_bracket_service_hours_right_square_bracket: None,
        }
    }
}

