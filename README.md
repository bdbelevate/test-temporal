# Testing Rust Temporal SDK

I don't think this is exactly the way to do it, but it works ;)

## How to run

```
# in one terminal, start temporal server
temporal server start-dev

# in another terminal, start the worker
cargo run

# in yet another terminal, send a client event
cargo run --bin client
```

## Todos

- [ ] figure out how to get the activity responses back to the client/workflow
- [ ] figure out how to await the workflow responses from the client
- [ ] make it easier to do
