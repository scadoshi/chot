use chot::{config::SOCKET_ADDR, streamer::Streamer};

fn main() -> anyhow::Result<()> {
    let stream = std::net::TcpStream::connect(SOCKET_ADDR)?;
    println!("Successfully connected to server");
    //println!("stream: {:?}", stream);
    Streamer::stream(stream)?;
    Ok(())
}
