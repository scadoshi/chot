# Chat Room Project - Async/Threading Learning

## Goal
Learn async/threading for interviews by building a chat app, step by step.

## Current Status: Step 2 - TCP Sockets

### What's Built
**Server** (`src/bin/server/main.rs`):
- ✅ Binds to 127.0.0.1:8080
- ✅ Accepts incoming connection
- ✅ Prints connection info (stream + socket_addr)
- ❌ Stream not used yet (just debug printed)

**Client** (`src/bin/client/main.rs`):
- ✅ Connects to 127.0.0.1:8080
- ✅ Prints success message
- ❌ Stream not used yet (just debug printed)

### What Works
- Run `cargo run --bin server` → waits for connection
- Run `cargo run --bin client` → connects successfully
- Both print confirmation messages

### Next Steps
1. Send/receive data over TcpStream (read/write)
2. Make it bidirectional (threads for reading)
3. Send messages back and forth

## Completed Steps
**Step 1: Channels** - Two threads communicating via `std::sync::mpsc::channel`

## Future Steps
- Multi-client support
- Add async with Tokio
- Build actual chat room with broadcast
