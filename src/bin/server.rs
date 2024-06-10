use wtransport::connection;
use wtransport::tls::Sha256DigestFmt;
use wtransport::Endpoint;
use wtransport::Identity;
use wtransport::ServerConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let identity = &Identity::self_signed(["localhost", "127.0.0.1"]).unwrap();
    let config = ServerConfig::builder()
        .with_bind_default(4433)
        .with_identity(identity)
        .build();

    let hash = identity.certificate_chain().as_slice()[0]
        .hash()
        .fmt(Sha256DigestFmt::BytesArray);
    println!("{}", hash);

    let server = Endpoint::server(config)?;

    let connection = server.accept().await;

    println!("accepted");
    let connection = connection.await?;
    println!("session req");
    let connection = connection.accept().await?;

    println!("connection established");

    let (mut send_stream, mut recv_stream) = connection.accept_bi().await?;
    let mut buffer = vec![0; 65536].into_boxed_slice();
    let bytes_read = recv_stream.read(&mut buffer).await?.unwrap();
    let str_data = std::str::from_utf8(&buffer[..bytes_read])?;
    println!("{str_data}");

    send_stream.write_all(&buffer).await?;

    Ok(())
}
