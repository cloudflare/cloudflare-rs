use crate::framework::response::ApiResult;
use crate::framework::Environment;
use serde::Serialize;
use url::Url;

pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

pub trait Endpoint<ResultType = (), QueryType = (), BodyType = ()>
where
    ResultType: ApiResult,
    QueryType: Serialize,
    BodyType: Serialize,
{
    fn method(&self) -> Method;
    fn path(&self) -> String;
    fn query(&self) -> Option<QueryType> {
        None
    }
    fn body(&self) -> Option<BodyType> {
        None
    }
    fn url(&self, environment: &Environment) -> Url {
        Url::from(environment).join(&self.path()).unwrap()
    }
    fn content_type(&self) -> String {
        "application/json".to_owned()
    }
}
