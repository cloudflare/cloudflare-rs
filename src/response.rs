extern crate reqwest;
extern crate serde_json;

use serde::de::DeserializeOwned;
use std::fmt::Debug;


#[derive(Deserialize, Debug)]
pub struct APISuccessResponse<ResultType> {
    result: ResultType,
    result_info: Option<serde_json::value::Value>,
    messages: serde_json::value::Value,
}

#[derive(Debug)]
pub struct APIFailureResponse {
    error: Option<reqwest::Error>,
    api_errors: Option<serde_json::value::Value>,
}

impl From<reqwest::Error> for APIFailureResponse {
    fn from(error: reqwest::Error) -> Self {
        APIFailureResponse{
            error: Some(error), 
            api_errors: None,
        }
    }
}

pub trait APIResult: DeserializeOwned + Debug{}

pub type APIResponse<ResponseType> = Result<APISuccessResponse<ResponseType>, APIFailureResponse>;


