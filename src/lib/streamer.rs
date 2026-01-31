use std::{
    io::{stdin, stdout, BufRead, BufReader, Write},
    net::TcpStream,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

pub struct Streamer;
impl Streamer {
    pub fn stream(stream: TcpStream) -> anyhow::Result<()> {
        let mut writer = stream.try_clone()?;
        stream.set_read_timeout(Some(Duration::from_millis(100)))?;
        let mut reader = BufReader::new(stream);
        let reader_end_handle = Arc::new(AtomicBool::new(false));
        let writer_end_handle = reader_end_handle.clone();
        // reader
        let reader_thread = thread::spawn(move || -> anyhow::Result<()> {
            loop {
                let mut line = String::new();
                match reader.read_line(&mut line) {
                    Ok(0) => break,
                    Ok(content) => {
                        if reader_end_handle.load(Ordering::Relaxed)
                            || content == 0
                            || line.trim() == "/exit"
                        {
                            println!("They exited");
                            reader_end_handle.store(true, Ordering::Relaxed);
                            break;
                        }
                        println!("Received: {}", line.trim());
                        stdout().flush()?;
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        if reader_end_handle.load(Ordering::Relaxed) {
                            break;
                        }
                        continue;
                    }
                    Err(e) => return Err(e.into()),
                }
            }
            Ok(())
        });
        // writer
        let writer_thread = thread::spawn(move || -> anyhow::Result<()> {
            'main: loop {
                println!("Type a message and click enter to send");
                for line in stdin().lines() {
                    if writer_end_handle.load(Ordering::Relaxed) {
                        println!("They exited");
                        break 'main;
                    }
                    let line = line?;
                    writer.write_all(line.as_bytes())?;
                    writer.write_all(b"\n")?;
                    writer.flush()?;
                    println!("Message sent");
                    if line.trim() == "/exit" {
                        println!("Exiting");
                        writer_end_handle.store(true, Ordering::Relaxed);
                        break 'main;
                    }
                }
            }
            Ok(())
        });
        reader_thread.join().expect("Reader thread failed")?;
        writer_thread.join().expect("Writer thread failed")?;
        Ok(())
    }
}
