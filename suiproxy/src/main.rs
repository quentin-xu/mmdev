use sui_client::Client as SuiClient;
use sui_service::SuiService;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let client = SuiClient::new("https://fullnode.devnet.sui.io:443".to_string());
    let service = SuiService::new(client);
    
    Server::builder()
        .add_service(sui_service::register_service(service).await?)
        .serve(addr)
        .await?;
    
    Ok(())
}
