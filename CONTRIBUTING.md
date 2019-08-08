Every PR should have a corresponding issue, and the issue number should appear in the PR's description.

PRs should be squashed to one commit before merging.

There are two root-level modules in `cloudflare-rs`. Most PRs will only touch one of them. If you're
working on the API framework itself, all the relevant code lives under the `framework/`
module. If you're looking to add or edit an endpoint, read on.

# Adding New Endpoints

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
