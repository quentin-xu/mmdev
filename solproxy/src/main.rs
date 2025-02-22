use tonic::{transport::Server, Request, Response, Status};
use solproxy::sol_proxy_server::{SolProxy, SolProxyServer};
use solproxy::{HelloReply, HelloRequest, SubmitTransactionRequest, GetTransactionRequest, TransactionResponse};
use client::SolClient;

pub mod solproxy {
    tonic::include_proto!("solproxy");
}

pub mod client;
pub mod transaction;

#[derive(Debug, Default)]
pub struct MySolProxy {
    client: SolClient,
}

#[tonic::async_trait]
impl SolProxy for MySolProxy {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }

    async fn submit_transaction(
        &self,
        request: Request<SubmitTransactionRequest>,
    ) -> Result<Response<TransactionResponse>, Status> {
        let msg = request.into_inner();
        let signature = self.client.submit_transaction(&msg.reciever, msg.amount).map_err(|e| {
            Status::internal(format!("Failed to submit transaction: {}", e))
        })?;
        Ok(Response::new(TransactionResponse {
            signature,
        }))
    }

    async fn get_transaction(
        &self,
        request: Request<GetTransactionRequest>,
    ) -> Result<Response<TransactionResponse>, Status> {
        let msg = request.into_inner();
        let tx = self.client.get_transaction(&msg.signature).map_err(|e| {
            Status::internal(format!("Failed to get transaction: {}", e))
        })?;
        Ok(Response::new(TransactionResponse {
            signature: transaction::get_signature(&tx)[0].clone()
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let proxy = MySolProxy::default();

    println!("SolProxyServer listening on {}", addr);

    Server::builder()
        .add_service(SolProxyServer::new(proxy))
        .serve(addr)
        .await?;

    Ok(())
}
