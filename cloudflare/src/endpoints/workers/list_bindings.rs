use super::WorkersBinding;
use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiSuccess;

/// List Bindings
/// Lists all bindings for a given script
#[derive(Debug)]
pub struct ListBindings<'a> {
    /// account id of owner of the script
    pub account_id: &'a str,
    /// name of script to list bindings for
    pub script_name: &'a str,
}

impl<'a> EndpointSpec for ListBindings<'a> {
    type JsonResponse = Vec<WorkersBinding>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts/{}/bindings",
            self.account_id, self.script_name
        )
    }
}
