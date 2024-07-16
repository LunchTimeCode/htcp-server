use std::{env, net::TcpStream};

#[derive(Debug, Clone)]
pub struct ClientFactory {
    pub target_binding: String,
}
impl ClientFactory {
    pub fn new_from_env() -> Self {
        let target_port: String = env::var("SERVER_PORT").unwrap_or("3001".to_string());
        let target_binding = format!("0.0.0.0:{}", target_port);
        println!("trying to connect to client at: {target_binding}");
        Self { target_binding }
    }

    pub async fn create_stream(&self) -> anyhow::Result<TcpStream> {
        let stream = TcpStream::connect(self.target_binding.clone())?;
        Ok(stream)
    }
}
