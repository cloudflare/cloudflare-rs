# Building New Client Modules

## TODO:

### add template directory to indicate usage:

* base directory ("/sample") should be named after the API in question.
* mod.rs should instantiate the internal mods, use them to make them public:
``` rust
mod do_something;

pub use do_something::DoSomething // where DoSomething is the Endpoint implementation.
```
* mod.rs should define common data structures for the module (i.e. Zone and its children). generally this includes implementation of the APIResult trait for top-level objects
* individual submodules should define an Endpoint per file. This should include all of the following (where appropriate):

	* Endpoint definition
	* request structs
	* response struct(s)

* submodules should include comment block describing endpoint usage.

### Document framework entities

document the Endpoint trait in terms of usage
document the pieces of an Endpoint trait in terms of usage

### Questions

question: where are error codes defined? should that happen here?
question: main lib is where all code is, suggestion would be to separate utility (framework) modules from user modules
question: how does this thing treat "non-standard" api responses; i.e. those responses that do not come back as the standard JSON object? in my framework, i wanted to coerce all responses into this format for consistency's sake, but this may be unwise.

## Suggested structure

extract crate for framework entities and shared mods (Endpoint, APIResponse, etc)
allow folks to publish their own clients, or publish them here?

tl;dr this feels like a framework. possible future concerns include:
* internal api nonsense
* possibility of versioning
