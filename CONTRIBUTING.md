# Contributing to cloudflare-rs

There are two root-level modules in `cloudflare-rs`. Most PRs will only touch one of them. The
`framework/` module contains the API framework itself. The `endpoints/` module contains code for
each Cloudflare API endpoint, grouped by product, e.g. `endpoints/dns` or `endpoints/zones`. Check 
the "Updating the Framework" or "Adding New Endpoints" sections below.

## Pull Requests

Every PR should have a corresponding issue, and the issue number should appear in the PR's 
description and commit message.

PRs should be squashed to one commit before merging.

## Updating the Framework

This library includes both async and blocking API clients. The `ApiClient` trait covers blocking 
requests, and is implemented by the `HttpApiClient` struct. The `async_api::Client` struct covers
async requests. Because Rust doesn't support async fns in traits yet, there is no trait for async
API clients.

If you want to change how the Cloudflare API client works, please remember to make the change in 
both blocking and async clients if applicable.

## Adding New Endpoints

Every Cloudflare product should have its own directory under `endpoints/`. For example, all the
DNS endpoints live under `endpoints/dns`.

If your product's module gets big enough, we suggest structuring it like so:

```
src/
    endpoints/
        myproduct/
            data_structures.rs
            endpoint_a.rs
            endpoint_b.rs
            mod.rs
```

In this structure, every endpoint gets its own module, which includes

 * Endpoint struct
 * Request struct
 * Response struct (if necessary)
 * Params struct (if necessary)

Common data structures which are used in multiple endpoints should be put in `data_structures.rs`.
`mod.rs` should then make all its submodules public, like so:

```rust
mod data_structures;
mod endpoint_a;
mod endpoint_b;

pub use data_structures;
pub use endpoint_a;
pub use endpoint_b;
```

## Documentation

Endpoint structs should have a docstring with a link to the [Cloudflare API docs](https://api.cloudflare.com).

Fields which represent endpoint parameters should be commented with their description in the
Cloudflare API docs.

If in doubt, follow the docstring structure for modules like `dns`. Ideally, someone reading your
endpoint code shouldn't need to open up api.cloudflare.com for documentation. Your comments should
be documentation enough.
