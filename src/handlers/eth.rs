use serde_json::json;

use crate::services::sqd_client::sqd_get_block_by_number;
use crate::services::sqd_utils::sqd_get_block_number;
use serde_json::Value;
/* // Request
curl -X POST --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":83}'
// Result
{
  "id":83,
  "jsonrpc": "2.0",
  "result": "0x4b7" // 1207
}
 */

pub async fn get_block_number() -> Result<serde_json::Value, anyhow::Error> {
    let block_number = sqd_get_block_number().await?;
    //convert blocknumber to hex number
    let hex_block_number = format!("0x{:x}", block_number);

    Ok(json!({ "id":1,"jsonrpc": "2.0", "result": hex_block_number }))
}

pub async fn get_block_by_number(block_number: u64) -> Result<Value, anyhow::Error> {
    let block = sqd_get_block_by_number(block_number).await?;
    Ok(json!({ "id":1,"jsonrpc": "2.0", "result": block }))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_get_block_number() {
        let block_number = get_block_number().await.unwrap();
        println!("hex_block_number: {:?}", block_number);
        assert!(block_number.is_object());
    }

    #[tokio::test]
    async fn test_get_block_by_number() {
        let block_number = 1000000;
        let block = get_block_by_number(block_number).await.unwrap();
        assert!(block.is_object());
    }
}
