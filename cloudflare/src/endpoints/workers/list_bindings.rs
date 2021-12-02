use super::WorkersBinding;
use crate::framework::endpoint::{Endpoint, Method};

/// List Bindings
/// Lists all bindings for a given script
#[derive(Debug)]
pub struct ListBindings<'a> {
    /// account id of owner of the script
    pub account_id: &'a str,
    /// name of script to list bindings for
    pub script_name: &'a str,
}

impl<'a> Endpoint<Vec<WorkersBinding>> for ListBindings<'a> {
    fn method(&self) -> Method {
        Method::Get
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts/{}/bindings",
            self.account_id, self.script_name
        )
    }
}
