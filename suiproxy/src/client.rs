use tonic::Request;
use su::sui_service::sui_client::SuiServiceClient;

pub async fn submit_transaction(
    url: &str,
    sender: &str,
    recipient: &str,
    amount: u64,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut client = SuiServiceClient::connect(format!("http://{}", url))
        .await?;
    
    let request = SubmitTransactionRequest {
        sender: sender.to_string(),
        recipient: recipient.to_string(),
        amount,
    };

    let response = client
        .submit_transaction(Request::new(request))
        .await?;
    
    Ok(response.into_inner().transaction_digest)
}

pub async fn get_transaction(
    url: &str,
    digest: &str,
) -> Result<(String, String, u64), Box<dyn std::error::Error>> {
    let mut client = SuiServiceClient::connect(format!("http://{}", url))
        .await?;
    
    let request = GetTransactionRequest {
        transaction_digest: digest.to_string(),
    };

    let response = client
        .get_transaction(Request::new(request))
        .await?;
    
    let resp = response.into_inner();
    Ok((resp.timestamp, resp.sender, resp.total_gas))
}

pub async fn get_account_balance(
    url: &str,
    address: &str,
) -> Result<u64, Box<dyn std::error::Error>> {
    let mut client = SuiServiceClient::connect(format!("http://{}", url))
        .await?;
    
    let request = GetAccountBalanceRequest {
        address: address.to_string(),
    };

    let response = client
        .get_account_balance(Request::new(request))
        .await?;
    
    Ok(response.into_inner().balance)
}