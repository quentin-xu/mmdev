use sui_client::{types::SuiAddress, Client as SuiClient};
use tonic::{Request, Response, Status};

pub struct SuiService {
    client: SuiClient,
}

impl SuiService {
    pub fn new(client: SuiClient) -> Self {
        Self { client }
    }
}

#[tonic::async_trait]
pub trait SuiServiceTrait {
    async fn submit_transaction(
        &self,
        request: Request<SubmitTransactionRequest>,
    ) -> Result<Response<SubmitTransactionResponse>, Status>;
    
    async fn get_transaction(
        &self,
        request: Request<GetTransactionRequest>,
    ) -> Result<Response<GetTransactionResponse>, Status>;
    
    async fn get_account_balance(
        &self,
        request: Request<GetAccountBalanceRequest>,
    ) -> Result<Response<GetAccountBalanceResponse>, Status>;
}

#[tonic::async_trait]
impl SuiServiceTrait for SuiService {
    async fn submit_transaction(
        &self,
        request: Request<SubmitTransactionRequest>,
    ) -> Result<Response<SubmitTransactionResponse>, Status> {
        let req = request.into_inner();
        let transaction = self.client.transfer(&req.sender, &req.recipient, req.amount).await?;
        Ok(Response::new(SubmitTransactionResponse {
            transaction_digest: transaction.digest.to_string(),
        }))
    }

    async fn get_transaction(
        &self,
        request: Request<GetTransactionRequest>,
    ) -> Result<Response<GetTransactionResponse>, Status> {
        let req = request.into_inner();
        let tx = self.client.get_transaction(&req.digest).await?;
        Ok(Response::new(GetTransactionResponse {
            timestamp: tx.timestamp.unwrap_or_default().to_string(),
            sender: tx.sender.to_string(),
            total_gas: tx.total_gas,
        }))
    }

    async fn get_account_balance(
        &self,
        request: Request<GetAccountBalanceRequest>,
    ) -> Result<Response<GetAccountBalanceResponse>, Status> {
        let req = request.into_inner();
        let balance = self.client.get_account_balance(&req.address).await?;
        Ok(Response::new(GetAccountBalanceResponse { balance }))
    }
}