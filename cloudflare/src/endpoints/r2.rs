use chrono::offset::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiResult;

/// A Bucket is a collection of Objects stored in R2.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Bucket {
    /// Bucket name
    pub name: String,
    /// Creation date of the bucket
    pub creation_date: DateTime<Utc>,
}

/// ListBucketsResult contains a list of buckets in an account.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ListBucketsResult {
    pub buckets: Vec<Bucket>,
}

type EmptyMap = HashMap<(), ()>;
impl ApiResult for EmptyMap {}
impl ApiResult for ListBucketsResult {}

/// Lists all buckets within the account.
#[derive(Debug)]
pub struct ListBuckets<'a> {
    pub account_identifier: &'a str,
}

impl<'a> EndpointSpec<ListBucketsResult> for ListBuckets<'a> {
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!("accounts/{}/r2/buckets", self.account_identifier)
    }
}

/// Creates a bucket with the given name.
/// A 400 is returned if the account already owns a bucket with this name.
/// A bucket must be explicitly deleted to be replaced.
#[derive(Debug)]
pub struct CreateBucket<'a> {
    pub account_identifier: &'a str,
    pub bucket_name: &'a str,
}

impl<'a> EndpointSpec<EmptyMap> for CreateBucket<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/r2/buckets/{}",
            self.account_identifier, self.bucket_name
        )
    }
}

/// Deletes a bucket with the given name.
#[derive(Debug)]
pub struct DeleteBucket<'a> {
    pub account_identifier: &'a str,
    pub bucket_name: &'a str,
}

impl<'a> EndpointSpec<EmptyMap> for DeleteBucket<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/r2/buckets/{}",
            self.account_identifier, self.bucket_name
        )
    }
}
