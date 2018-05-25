# General

This document is kept to list technical concerns the token team will want to address in libsovtoken (or indy-sdk
where this libraries interact).

# Issues

# Issue -- RESOLVED: circular references between indy-sdk and libsovtoken
See each of the below sub-items for proposed solutions

### APIs return type 'ErrorCode'
> `Proposed solution`:  have the APIs return i32 for codes.  0 always means success.  Anything else is value that means
something to the plugin

### Circular build references vs Duplication of code
> `Proposed solution`:  put shared code into a separate library/crate and have indy-sdk and libsovtoken reference on the
shared library


### Resolution
This issue is resolved by using rust-indy-sdk