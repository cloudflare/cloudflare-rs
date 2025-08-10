use crate::api_endpoint;
use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::{ApiResult, ApiSuccess};
use serde::{Deserialize, Serialize};

use super::types::Queue;

// INFO: ref: https://developers.cloudflare.com/api/resources/queues/subresources/purge/

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct ConfirmDelete {
    /// Confimation that all messages will be deleted permanently.
    delete_messages_permanently: bool,
}

api_gen!(ConfirmDelete);

api_endpoint! (
    /// Purge Queue
    /// Deletes all messages from the Queue.
    /// https://developers.cloudflare.com/api/resources/queues/subresources/purge/
    POST,
    PurgeQueue => Queue,
    "accounts/{}/queues/{}/purge";
    account_id,
    queue_id;
    params: ConfirmDelete
);

api_results!(QueuePurgeStatus {
    /// Indicates if the last purge operation completed successfully.
    completed: String,
    /// Timestamp when the last purge operation started.
    started_at: String
});
