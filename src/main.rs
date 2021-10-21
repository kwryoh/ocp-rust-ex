use anyhow::Result;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return ,
                    Ok(n) => n,
                    Err(e) => {
                        eprint!("failed to read from socket; err = {:?}", e);
                        return ;
                    }
                };

                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return ;
                }
            }
        });
    }
}
