use chot::{config::SOCKET_ADDR, streamer::Streamer};

fn main() -> anyhow::Result<()> {
    let listener = std::net::TcpListener::bind(SOCKET_ADDR)?;
    println!("Server listening on {}", SOCKET_ADDR);
    let (stream, _socket_addr) = listener.accept()?;
    println!("Client successfully connected");
    //println!("stream: {:?}", stream);
    //println!("socket_addr: {:?}", socket_addr);
    Streamer::stream(stream)?;
    Ok(())
}
