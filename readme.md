
## typestate drill

- `connect` technicaly should return Yes or No, which means it should be a `Result<Session<Connecting>, Session<Disconnected>>`, it's forcing clients to `match`
- To share `Session<Ready>` it has to be in `Arc<Mutex<Session<Ready>>>`. Assume Thread A and Thread B sharing this and Thread B want to disconnect there is no good answer.
- Reconnecting under the hood is hard because user already has `Session<Ready>`, we can't force it back to `Session<Connecting>`


## actor pattern drill

- `&current_state` in `match (&current_state, cmd) {}` is an immutable borrow, however it's dropped after match allowing `current_state = InternalState::Connecting`. This is because of [Non-lexical lifetimes)[https://blog.rust-lang.org/2022/08/05/nll-by-default/]
- Now `spawn_client` is actually the big switch statement. We'll see how this is fixed

