use std::{
    io::{stdin, stdout, BufRead, BufReader, Write},
    net::TcpStream,
    thread,
};

pub struct Streamer;
impl Streamer {
    pub fn stream(stream: TcpStream) -> anyhow::Result<()> {
        let mut writer = stream.try_clone()?;
        let mut reader = BufReader::new(stream);
        // reader
        let reader_thread = thread::spawn(move || -> anyhow::Result<()> {
            loop {
                let mut line = String::new();
                if reader.read_line(&mut line)? == 0 {
                    break;
                }
                println!("-> {}", line);
                stdout().flush().ok();
            }
            Ok(())
        });
        // writer
        let writer_thread = thread::spawn(move || -> anyhow::Result<()> {
            'main: loop {
                for line in stdin().lines() {
                    let line = line?;
                    if line.trim() == "/exit" {
                        println!("Goodbye!");
                        break 'main;
                    }
                    writer.write_all(line.as_bytes())?;
                    writer.write_all(b"\n")?;
                    writer.flush()?;
                }
            }
            Ok(())
        });
        reader_thread.join().expect("Reader thread failed")?;
        writer_thread.join().expect("Writer thread failed")?;
        Ok(())
    }
}
