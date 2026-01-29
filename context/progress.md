# Chat Room Project - Async/Threading Learning

## Goal
Learn async/threading for interviews by building a chat app, step by step.

## Current Status: Step 3 - Multi-client Support

### What's Built
**Server** (`src/bin/server/main.rs`):
- ✅ Binds to 127.0.0.1:8080
- ✅ Accepts incoming connection
- ✅ Passes to `Streamer::stream()`

**Client** (`src/bin/client/main.rs`):
- ✅ Connects to 127.0.0.1:8080
- ✅ Passes to `Streamer::stream()`

**Streamer** (`src/lib/streamer.rs`):
- ✅ Bidirectional communication (read/write threads)
- ✅ Send/receive messages over TcpStream
- ✅ Read from stdin, write to network
- ✅ Read from network, print to stdout

### What Works
- Run `cargo run --bin server` → waits for connection
- Run `cargo run --bin client` → connects and sends/receives messages
- Type messages, press Enter → sent to other side
- Type `/exit` → closes connection

### Next Steps (TODO)
1. Make `/exit` cleanly exit both programs
2. Allow server to accept new client after first disconnects
3. Support multiple simultaneous clients

## Completed Steps
**Step 1: Channels** - Two threads communicating via `std::sync::mpsc::channel`
**Step 2: TCP Sockets** - Bidirectional messaging with TcpStream and threads

## Future Steps
- Multi-client support
- Add async with Tokio
- Build actual chat room with broadcast
