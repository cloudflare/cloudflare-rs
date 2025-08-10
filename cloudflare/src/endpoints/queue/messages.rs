use crate::api_endpoint;
use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::{ApiResult, ApiSuccess};

use super::messages_types::{AckQueueResults, ActionMessage, PullQueueResults};

// INFO: ref. https://developers.cloudflare.com/api/resources/queues/subresources/messages/

api_gen!(AckQueueResults);
api_endpoint!(
    /// Acknowledge + Retry messages from a Queue.
    /// <https://developers.cloudflare.com/api/resources/queues/subresources/messages/methods/ack/>
    POST,
    AckRetry => AckQueueResults,
    "accounts/{}/queues/{}/messages/ack";
    account_id,
    queue_id;
    params: ActionMessage
);

api_gen!(PullQueueResults);
api_endpoint!(
    /// Pull a batch of messages from a Queue.
    /// <https://developers.cloudflare.com/api/resources/queues/subresources/messages/methods/pull/>
    POST,
    PullQueue => PullQueueResults,
    "/accounts/{}/queues/{}/messages/pull";
    account_id,
    queue_id
);
