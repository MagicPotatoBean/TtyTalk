use std::net::{SocketAddr, SocketAddrV4};

#[derive(Debug)]
pub enum AppMode {
    Visual,
    Type(String),
    Connect(String),
}
impl Default for AppMode {
    fn default() -> Self {
        Self::Visual
    }
}
impl AppMode {
    pub fn force_type(&mut self) -> Option<&mut String> {
        if let AppMode::Type(ref mut data) = self {
            Some(data)
        } else {
            None
        }
    }
    pub fn force_connect(&mut self) -> Option<&mut String> {
        if let AppMode::Connect(ref mut data) = self {
            Some(data)
        } else {
            None
        }
    }
}
#[derive(Debug, Default)]
pub struct AppState {
    pub mode: AppMode,
    pub connection: Option<net_message::asymmetric::AsymmetricTcpStream<ClientToServerMsg, ServerToClientMsg>>,
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
