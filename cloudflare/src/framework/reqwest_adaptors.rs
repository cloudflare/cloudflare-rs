use crate::framework::endpoint::Method;

pub fn match_reqwest_method(method: Method) -> reqwest::Method {
    match method {
        Method::Get => reqwest::Method::GET,
        Method::Post => reqwest::Method::POST,
        Method::Delete => reqwest::Method::DELETE,
        Method::Put => reqwest::Method::PUT,
        Method::Patch => reqwest::Method::PATCH,
    }
}
