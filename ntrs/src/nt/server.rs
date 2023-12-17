use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpSocket, TcpStream};

use tokio;

use std::error::Error;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::str::FromStr;

use crate::nt::NTConfig;

pub struct NT {
    config: NTConfig,
}

impl NT {
    pub fn new(config: NTConfig) -> NT {
        NT { config }
    }

    pub async fn start(&mut self) -> Result<bool, Box<dyn Error>> {
        let listener = TcpListener::bind(format!("{}:{}", self.config.host, self.config.port))
            .await
            .unwrap();
        println!("NT listening with {:?}", self.config);

        loop {
            let (mut stream, socket_addr) = listener.accept().await.unwrap();
            if let Ok(_) = self.process(&mut stream, &socket_addr).await {
                continue;
            } else {
                break Ok(false);
            }
        }
    }

    async fn process(
        &mut self,
        ingress: &mut TcpStream,
        socket: &SocketAddr,
    ) -> Result<bool, Box<dyn Error>> {
        let src_string = socket.to_string();
        let src = src_string.split(":").next().unwrap();

        let check_dest = self.config.get_dest(String::from(src));
        let dest: String;
        if let Some(addr) = check_dest {
            dest = addr;
            println!("Connection {} -> {}", &src, dest);
        } else {
            println!("{} not in paths. Ignoring.", &socket);
            return Ok(false);
        }

        let egress_sock = TcpSocket::new_v4().unwrap();
        let egress_ip = Ipv4Addr::from_str(dest.as_str()).unwrap();
        let egress_addr = SocketAddrV4::new(egress_ip, 0);
        let mut egress: TcpStream;

        if let Ok(egress_stream) = egress_sock.connect(SocketAddr::V4(egress_addr)).await {
            egress = egress_stream;
        } else {
            println!("Dest refused connection.");
            ingress.shutdown().await?;
            return Ok(false);
        }

        match tokio::io::copy_bidirectional(ingress, &mut egress).await {
            Ok((bytes_ingress, bytes_egress)) => {
                println!(
                    "Ingress {} bytes, Egress {} bytes",
                    bytes_ingress, bytes_egress
                );
                return Ok(true);
            }
            _ => return Ok(false),
        }
    }
}
