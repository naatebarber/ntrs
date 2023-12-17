use std::{time::Duration, net::{SocketAddr, SocketAddrV4, Ipv4Addr}, str::FromStr};
use tokio::{self, io::{AsyncReadExt, AsyncWriteExt}, join};



#[tokio::main]
async fn main() {
    let frequency_seconds = Duration::from_millis(1000);

    let prod_sock = tokio::net::TcpSocket::new_v4().unwrap();
    let cons_sock = tokio::net::TcpSocket::new_v4().unwrap();

    let prod_h = tokio::spawn(producer(prod_sock, frequency_seconds));

    let cons_h = tokio::spawn(consumer(cons_sock));

    let (p, c) = join!(prod_h, cons_h);
    p.unwrap();
    c.unwrap();
}

async fn producer(sock: tokio::net::TcpSocket, freq: Duration) {
    let addr = SocketAddrV4::new(Ipv4Addr::from_str("127.0.0.1").unwrap(), 5000);

    let mut egress = sock.connect(SocketAddr::V4(addr)).await.unwrap();

    loop {
        tokio::time::sleep(freq).await;

        let message = String::from("hello");

        egress.writable().await.unwrap();

        match egress.write(message.as_bytes()).await {
            Err(e) => println!("[Producer] failed to write {:?}", e),
            _ => ()
        }
    }

}

async fn consumer(sock: tokio::net::TcpSocket) {
    let addr = SocketAddrV4::new(Ipv4Addr::from_str("127.0.0.1").unwrap(), 5000);

    sock.bind(SocketAddr::V4(addr)).unwrap();
    let listener = sock.listen(8).unwrap();

    let mut buf: Vec<u8> = vec![];

    loop {
        match String::from_utf8(buf.clone()) {
            Ok(s) => println!("{}", s),
            Err(_) => println!("[Consumer] parse err")
        }

        let (mut stream, _) = listener.accept().await.unwrap();

        println!("Accepted a connection");

        stream.read(&mut buf).await.unwrap();
    }
}