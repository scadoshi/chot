fn main() -> anyhow::Result<()> {
    let addr = "127.0.0.1:8080";
    let listener = std::net::TcpListener::bind(addr)?;
    println!("Server listening on {:?}", addr);
    let (tcp_stream, socket_addr) = listener.accept()?;
    println!("Client successfully connected");
    println!("tcp_stream: {:?}", tcp_stream);
    println!("socket_addr: {:?}", socket_addr);
    Ok(())
}
