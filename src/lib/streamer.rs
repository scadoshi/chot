use std::{
    io::{stdin, stdout, BufRead, BufReader, Write},
    net::TcpStream,
    thread,
    time::Duration,
};

pub struct Streamer;
impl Streamer {
    pub fn stream(stream: TcpStream) -> anyhow::Result<()> {
        let mut writer = stream.try_clone()?;
        stream.set_read_timeout(Some(Duration::from_millis(100)))?;
        let mut reader = BufReader::new(stream);
        let (ender, end_signal) = std::sync::mpsc::channel::<bool>();
        // reader
        let reader_thread = thread::spawn(move || -> anyhow::Result<()> {
            loop {
                let mut should_end = false;
                if let Ok(signal) = end_signal.try_recv() {
                    should_end = signal;
                }
                let mut line = String::new();
                let content = reader.read_line(&mut line)?;
                if should_end || content == 0 || line.trim() == "/exit" {
                    println!("They exited");
                    break;
                }
                println!("Received: {}", line.trim());
                stdout().flush()?;
            }
            Ok(())
        });
        // writer
        let writer_thread = thread::spawn(move || -> anyhow::Result<()> {
            'main: loop {
                println!("Type a message and click enter to send");
                for line in stdin().lines() {
                    let line = line?;
                    if line.trim() == "/exit" {
                        println!("Exiting");
                        ender.send(true)?;
                        break 'main;
                    }
                    writer.write_all(line.as_bytes())?;
                    writer.write_all(b"\n")?;
                    writer.flush()?;
                    println!("Message sent");
                }
            }
            Ok(())
        });
        reader_thread.join().expect("Reader thread failed")?;
        writer_thread.join().expect("Writer thread failed")?;
        Ok(())
    }
}
