use response::APIResult;
use url::Url;
use serde::Serialize;
use super::Environment;


pub enum Method {
	Get,
	Post,
	Put,
	Delete,
	Patch,
}

pub trait Endpoint<ResultType, QueryType = (), BodyType = ()>
    where ResultType: APIResult,
          QueryType: Serialize,
          BodyType: Serialize {

    fn method(&self) -> Method;
    fn path(&self) -> String;
    fn query(&self) -> Option<QueryType> { None }
    fn body(&self) -> Option<BodyType> { None }

    fn url(&self, environment: &Environment) -> Url {
        Url::from(environment).join(&self.path()).unwrap()
    }
}
