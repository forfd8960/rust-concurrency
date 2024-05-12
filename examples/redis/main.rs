use std::{io, net::SocketAddr};

use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};
use tracing::{info, warn};

const BUF_SIZE: usize = 1024;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    println!("dummy redis server...");

    let addr = "0.0.0.0:6379";
    let listner = TcpListener::bind(addr).await?;
    info!("listen on: {}", addr);

    loop {
        let (conn, recv_addr) = listner.accept().await?;
        info!("accept conn from: {}", recv_addr);

        tokio::spawn(async move {
            if let Err(e) = process_redis_conn(conn, recv_addr).await {
                warn!("process_redis_conn error: {}", e);
            }
        });
    }
}

async fn process_redis_conn(mut conn: TcpStream, addr: SocketAddr) -> anyhow::Result<()> {
    // let (reader, writer) = conn.into_split();

    // let mut buf_reader = BufReader::new(reader);
    // let mut buf_writer = BufWriter::new(writer);

    loop {
        conn.readable().await?;

        let mut buf: Vec<_> = Vec::with_capacity(BUF_SIZE);
        match conn.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                info!("read {} bytes", n);
                let line = String::from_utf8_lossy(&buf[..n]);
                info!("read: {:?}", line);
                conn.write_all(b"+OK\r\n").await?
            }

            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }

            Err(e) => {
                return Err(e.into());
            }
        }
    }

    warn!("Connection {} closed", addr);
    anyhow::Ok(())
}
