use std::net::{SocketAddr, TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:65432").unwrap();
    loop {
        let client = listener.accept().unwrap();
        println!("Connected to {}", client.1);
        let mut stream = net_message::asymmetric::AsymmetricTcpStream::<ServerToClientMsg, ClientToServerMsg>::new(client.0).unwrap();
        while let Ok(msg) = stream.read() {
            println!("msg = {msg:?}");
        }
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize, type_hash::TypeHash)]
pub enum ClientToServerMsg {
    Message(String),
    File(Vec<u8>),
}
#[derive(Debug, serde::Serialize, serde::Deserialize, type_hash::TypeHash)]
pub enum ServerToClientMsg {
    Message(SocketAddr, String),
    File(SocketAddr, Vec<u8>),
}
