use crate::api_endpoint;
use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::{ApiResult, ApiSuccess};

use super::consumer_types::Consumer;

// INFO: ref. https://developers.cloudflare.com/api/resources/queues/subresources/consumers/

api_gen!(Consumer);
api_endpoint!(
    /// Create A Queue Consumer
    /// Creates a new consumer for a Queue.
    /// <https://developers.cloudflare.com/api/resources/queues/subresources/consumers/methods/create/>
    POST,
    CreateQueueConsumer => Consumer,
    "accounts/{}/queues/{}/consumers";
    account_id,
    queue_id;
    params: Consumer
);

api_endpoint!(
    /// Delete Queue Consumer
    /// Deletes the consumer for a queue.
    /// <https://developers.cloudflare.com/api/resources/queues/subresources/consumers/methods/delete/>
    DELETE,
    DeleteQueueConsumer => (),
    "accounts/{}/queues/{}/consumers/{}";
    account_id,
    queue_id,
    consumer_id
);

api_gen!(Vec<Consumer>);

api_endpoint!(
    /// List Queue Consumers
    /// Returns the consumers for a Queue.
    /// <https://developers.cloudflare.com/api/resources/queues/subresources/consumers/methods/get/>
    GET,
    ListQueueConsumer => Vec<Consumer>,
    "accounts/{}/queues/{}/consumers";
    account_id,
    queue_id
);

api_endpoint!(
    /// Update Queue Consumer
    /// Updates the consumer for a queue, or creates one if it does not exist..
    /// <https://developers.cloudflare.com/api/resources/queues/subresources/consumers/methods/update/>
    PUT,
    UpdateQueueConsumer => Consumer,
    "accounts/{}/queues/{}/consumers/{}";
    account_id,
    queue_id,
    consumer_id;
    params: Consumer
);
