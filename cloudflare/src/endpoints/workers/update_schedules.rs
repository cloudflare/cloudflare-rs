use serde::Deserialize;

use super::WorkersSchedule;

use crate::framework::{
    endpoint::{EndpointSpec, Method, RequestBody},
    response::{ApiResult, ApiSuccess},
};

/// Update Schedules
/// <https://developers.cloudflare.com/api/resources/workers/subresources/scripts/subresources/schedules/methods/update/>
#[derive(Debug)]
pub struct UpdateSchedules<'a> {
    /// Account ID of owner of the script
    pub account_identifier: &'a str,
    /// Name of the script, used in URLs and route configuration.
    pub script_name: &'a str,
    /// Schedules to be updated
    pub schedules: Vec<WorkersSchedule>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSchedulesResponse {
    pub schedules: Vec<WorkersSchedule>,
}

impl ApiResult for UpdateSchedulesResponse {}

impl EndpointSpec for UpdateSchedules<'_> {
    type JsonResponse = UpdateSchedulesResponse;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PUT
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts/{}/schedules",
            self.account_identifier, self.script_name
        )
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.schedules).unwrap();
        Some(RequestBody::Json(body))
    }
}
