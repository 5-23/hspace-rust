use chat::{get_event, timestamp, Event, User};
use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Mutex;
use std::{env, io};
use tokio::net::UdpSocket;

const ADDR: &'static str = "0.0.0.0:6163";

lazy_static::lazy_static! {
    /// (CHANNEL, PLAYER)
    static ref CLIENTS: Mutex<HashMap<String, (SocketAddr, User)>> = Mutex::new(HashMap::new());
}

struct Server {
    socket: UdpSocket,
    buf: Vec<u8>,
    to_send: Option<(usize, SocketAddr)>,
}

impl Server {
    async fn run(self) -> Result<(), io::Error> {
        let Server {
            socket,
            mut buf,
            mut to_send,
        } = self;

        loop {
            if let Some((size, peer)) = to_send {
                // let amt = socket.send_to(&buf[..size], &peer).await?;
                let buf = buf[..size].to_vec();
                let event = String::from_utf8(buf.clone()).unwrap();

                let _ = match get_event(&event) {
                    Event::Join(ip, name) => {
                        let mut clients = CLIENTS.lock().unwrap();
                        println!("{}", timestamp());
                        clients.insert(
                            ip.clone(),
                            (
                                peer,
                                User {
                                    ip: ip.clone(),
                                    name: name.clone(),
                                },
                            ),
                        );
                        for (_, value) in clients.iter_mut() {
                            socket
                                    .send_to(
                                        format!("{{\"type\": \"join\", \"ip\": \"{ip}\", \"name\": {name:?}}}")
                                            .as_bytes(),
                                        value.0,
                                    )
                                    .await
                                    .unwrap_or_else(|_| 1);
                        }
                    }
                    _ => {}
                };
                println!("{}: {:?}", peer, get_event(&event));
            }

            to_send = Some(socket.recv_from(&mut buf).await?);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args().nth(1).unwrap_or_else(|| ADDR.to_string());

    let socket = UdpSocket::bind(&addr).await?;
    println!("Listening on: {}", socket.local_addr()?);

    let server = Server {
        socket,
        buf: vec![0; 1024],
        to_send: None,
    };

    // This starts the server task.
    server.run().await?;

    Ok(())
}
