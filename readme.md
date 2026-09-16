
## typestate drill

- `connect` technicaly should return Yes or No, which means it should be a `Result<Session<Connecting>, Session<Disconnected>>`, it's forcing clients to `match`
- To share `Session<Ready>` it has to be in `Arc<Mutex<Session<Ready>>>`. Assume Thread A and Thread B sharing this and Thread B want to disconnect there is no good answer.
- Reconnecting under the hood is hard because user already has `Session<Ready>`, we can't force it back to `Session<Connecting>`

