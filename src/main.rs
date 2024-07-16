use server::create_server;

mod client;
mod server;

use colored::Colorize;

#[tokio::main]
async fn main() {
    match start().await {
        Ok(_) => println!("finished run with no errors"),
        Err(err) => println!("{}", err.to_string().red()),
    }
}

pub async fn start() -> anyhow::Result<()> {
    let client_factory = client::ClientFactory::new_from_env();
    let (listener, app) = create_server(client_factory).await?;

    axum::serve(listener, app).await.unwrap();
    Ok(())
}
