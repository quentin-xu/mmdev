use solana_transaction_status::{EncodedTransaction};
use std::{str::FromStr};

pub fn get_signature(tx: &EncodedTransaction) -> Vec<String> {
    match tx {
        EncodedTransaction::LegacyBinary(base58) => vec!(String::from_str("").unwrap()),
        EncodedTransaction::Binary(sig, data) => vec!(sig.to_string()),
        EncodedTransaction::Json(json) => json.signatures.clone(),
        EncodedTransaction::Accounts(accouts) => accouts.signatures.clone(),
    }
}

// fn parse_instructions(transaction_base58: &str) -> Result<(), Box<dyn Error>> {
//     // 将 Base58 编码的交易解码为字节
//     let transaction_data = bs58::decode(transaction_base58).into_vec()?;
    
//     // 将字节反序列化为 Transaction 对象
//     let transaction = Transaction::try_from(transaction_data.as_slice())?;
    
//     // 获取交易消息
//     let message = transaction.message();
    
//     // 遍历所有指令
//     for (i, instruction) in message.instructions().iter().enumerate() {
//         println!("Instruction {}:", i + 1);
        
//         // 获取程序 ID
//         let program_id = message.account_keys().get(instruction.program_id_index as usize)
//             .ok_or("Invalid program id index")?;
//         println!("  Program ID: {}", program_id);
        
//         // 解析账户
//         let accounts: Vec<&Pubkey> = instruction.accounts.iter()
//             .map(|&index| message.account_keys().get(index as usize))
//             .collect::<Option<Vec<_>>>()
//             .ok_or("Invalid account index")?;
        
//         println!("  Accounts:");
//         for account in accounts {
//             println!("    {}", account);
//         }
        
//         // 显示原始指令数据（Base64 编码）
//         println!("  Data (Base64): {}", base64::encode(&instruction.data));
//     }
    
//     Ok(())
// }
