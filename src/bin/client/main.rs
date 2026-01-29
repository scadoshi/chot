fn main() -> anyhow::Result<()> {
    let addr = "127.0.0.1:8080";
    let tcp_conn = std::net::TcpStream::connect(addr)?;
    println!("Successfully connected to server");
    println!("tcp_conn: {:?}", tcp_conn);
    Ok(())
}
