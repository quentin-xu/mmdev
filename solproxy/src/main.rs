use tonic::{transport::Server, Request, Response, Status};
use solproxy::sol_proxy_server::{SolProxy, SolProxyServer};
use solproxy::{HelloReply, HelloRequest};

pub mod solproxy {
    tonic::include_proto!("solproxy");
}

#[derive(Debug, Default)]
pub struct MySolProxy {}

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
