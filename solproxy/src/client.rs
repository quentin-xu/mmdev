use solana_sdk::{pubkey::Pubkey, signature::*, system_instruction, transaction::Transaction};
use solana_client::rpc_client::RpcClient;
use solana_transaction_status::{UiTransactionEncoding, EncodedTransaction, UiTransaction};
use std::{str::FromStr, *};

#[derive(Debug)]
pub struct SolClientError {
    message: String,
}

impl fmt::Display for SolClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Validation error: {}", self.message)
    }
}

impl error::Error for SolClientError {}

pub struct SolClient {
    rpc_client: RpcClient,
    sender: Keypair,
}

impl fmt::Debug for SolClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SolClient")
            .field("rpc_client", &"RpcClient")
            .finish()
    }
}

impl Default for SolClient {
    fn default() -> Self {
        let endpoint = &String::from("https://api.devnet.solana.com");
        let keypath = &String::from("~/.config/solana/id.json");
        SolClient::new(endpoint, keypath)
    }
}

impl SolClient {
    pub fn new(endpoint: &String, keypath: &String) -> Self {
        Self {
            rpc_client: RpcClient::new(endpoint),
            sender: keypair::read_keypair_file(keypath).unwrap(),
        }
    }

    pub fn submit_transaction(
        &self,
        receiver: &String,
        amount : u64
    ) -> Result<String, SolClientError> {
        let receiver = Pubkey::from_str(receiver).map_err(|e| {
            SolClientError { message: format!("Invalid receiver address: {}", e)}
        })?;
        let sender = self.sender.pubkey();
        
        let instruction = system_instruction::transfer(
            &sender,
            &receiver,
            amount,
        );

        let mut transaction = Transaction::new_with_payer(
            &[instruction],
            Some(&sender),
        );

        let recent_blockhash = self.rpc_client.get_latest_blockhash().map_err(|e| {
            SolClientError { message: format!("Failed to get recent blockhash: {}", e)}
        })?;

        transaction.sign(&[&self.sender], recent_blockhash);

        let signature = self.rpc_client.send_and_confirm_transaction(&transaction)
            .map_err(|e| 
                SolClientError { message: format!("Failed to send transaction: {}", e)}
            )?;

        Ok(signature.to_string())
    }

    pub fn get_transaction(
        &self,
        signature: &String,
    ) -> Result<UiTransaction, SolClientError> {
        let signature = signature.parse().map_err(|_| {
            SolClientError { message: format!("Invalid transaction signature format")}
        })?;

        let transaction = self.rpc_client.get_transaction(&signature, UiTransactionEncoding::Json)
            .map_err(|e| 
                SolClientError { message: format!("Failed to get transaction: {}", e)}
            )?;

        match transaction.transaction.transaction {
            EncodedTransaction::Json(tx) => {
                return Ok(tx)
            }
            _ => {
                return Err(SolClientError { message: format!("Invalid transaction format")})
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use mockito::Server;
    //use serde_json::json;

    fn create_test_client() -> SolClient {
        dotenv::dotenv().ok();
        let endpoint = &String::from("https://api.devnet.solana.com");
        let keypath = &std::env::var("SENDER_SECRET_KEY").expect("SENDER_SECRET_KEY must be set");
        println!("SENDER_SECRET_KEY:{}", keypath);
        SolClient::new(endpoint, keypath)
    }

    // #[test]
    // fn test_submit_transaction_success() {
    //     let client = create_test_client();
    //     let result = client.submit_transaction(&"test_receiver".to_string(), 100);
    //
    //     //mock_blockhash.assert();
    //     //mock_signature.assert();
    //     assert!(result.is_ok());
    // }

    // #[test]
    // fn test_submit_transaction_invalid_receiver() {
    //     let client = create_test_client();
    //     let result = client.submit_transaction(&"invalid".to_string(), 100);
    //
    //     assert!(result.is_err());
    //     assert!(result.unwrap_err().message.contains("Invalid receiver address"));
    // }

    #[test]
    fn test_get_transaction_success() {
        let client = create_test_client();
        
        // First submit a transaction to get a valid signature
        // let submit_result = client.submit_transaction(&"2xSvAsMb9JpHe3aMmMbUVvEh6YBrvdBrz8i6sL2d1pJ9bd".to_string(), 100);
        let signature = String::from("43sTgpdRyHV63ocC2KRqmY6mjvbUDZDBFAzV3sajBbPDPSMK37wykYAqNH7dtUcH3DUZN7GMfxR6PoKxdXWqf1Zy");
        
        // Now use the valid signature to test get_transaction()
        let result = client.get_transaction(&signature);
        
        assert!(result.is_ok());

        //println!("{}", result.unwrap())
    }

    #[test]
    fn test_get_transaction_invalid_signature() {
        let client = create_test_client();
        let result = client.get_transaction(&"invalid".to_string());
 
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "Invalid transaction signature format");
    }
}