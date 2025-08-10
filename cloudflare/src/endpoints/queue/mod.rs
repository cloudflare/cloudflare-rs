use crate::api_endpoint;
use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::{ApiResult, ApiSuccess};
pub mod consumer;
pub mod consumer_types;
pub mod messages;
pub mod messages_types;
pub mod purge;
pub mod types;
use types::*;

api_gen!(Queue);
api_endpoint!(
    /// Create a new queue.
    /// <https://developers.cloudflare.com/api/resources/queues/methods/create/>
    POST,
    CreateQueue => Queue,
    "accounts/{}/queues";
    account_id;
    params: Queue
);

api_endpoint!(
    /// Deletes a queue.
    /// <https://developers.cloudflare.com/api/resources/queues/methods/delete/>
    DELETE,
    DeleteQueue => (),
    "accounts/{}/queues/{}";
    account_id,
    queue_id
);

api_endpoint!(
    /// Edit a Queue.
    /// <https://developers.cloudflare.com/api/resources/queues/methods/edit/>
    PATCH,
    UpdateQueue => Queue,
    "accounts/{}/queues/{}";
    account_id,
    queue_id;
    params: Queue
);

api_endpoint!(
    /// Get details about a specific queue..
    /// <https://developers.cloudflare.com/api/resources/queues/methods/get/>
    GET,
    GetQueue => Queue,
    "accounts/{}/queues/{}";
    account_id,
    queue_id
);

api_gen!(Vec<Queue>);

api_endpoint!(
    /// Returns the queues owned by an account..
    /// <https://developers.cloudflare.com/api/resources/queues/methods/get/>
    GET,
    ListQueue => Vec<Queue>,
    "accounts/{}/queues";
    account_id
);
