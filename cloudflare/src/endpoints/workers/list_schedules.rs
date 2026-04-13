use serde::{Deserialize, Serialize};

use super::WorkersSchedule;

use crate::framework::{
    endpoint::{EndpointSpec, Method},
    response::{ApiResult, ApiSuccess},
};

/// List Schedules
/// <https://developers.cloudflare.com/api/resources/workers/subresources/scripts/subresources/schedules/methods/get/>
#[derive(Debug)]
pub struct ListSchedules<'a> {
    /// Account ID of owner of the script
    pub account_identifier: &'a str,
    /// The name of the script to list the schedules
    pub script_name: &'a str,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListSchedulesResponse {
    pub schedules: Vec<WorkersSchedule>,
}

impl ApiResult for ListSchedulesResponse {}

impl EndpointSpec for ListSchedules<'_> {
    type JsonResponse = ListSchedulesResponse;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts/{}/schedules",
            self.account_identifier, self.script_name
        )
    }
}
