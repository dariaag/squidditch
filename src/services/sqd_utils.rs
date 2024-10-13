use anyhow::{Error, Result};
use reqwest::Client;
use serde_json::{json, Value};
//TODO: fix env var reading
pub fn get_archive_url() -> Result<String, Error> {
    /* let url = env::var("SQD_URL");
    match url {
        Ok(url) => {
            println!("url: {:?}", url);
            Ok(url)
        }
        Err(_) => {
            println!("url: {:?}", url);
            Err(anyhow::anyhow!("SQD_URL is not set"))
        }
    } */
    Ok("https://v2.archive.subsquid.io/network/ethereum-mainnet".to_string())
}

// Get the current block number from SQD's API endpoint
pub async fn sqd_get_block_number() -> Result<u64, Error> {
    // Create an HTTP client
    let client = Client::new();
    let url = format!("{}/height", get_archive_url()?);
    // Make a request to SQD's API endpoint
    let response = client.get(url).send().await?;
    // Parse the response
    let block_number: u64 = response.json().await?;
    Ok(block_number)
}

pub async fn get_worker_url(block_number: u64) -> Result<String, Error> {
    let client = Client::new();
    //archive url
    let url = get_archive_url()?;
    let latest_block_number = sqd_get_block_number().await?;
    if latest_block_number < block_number {
        return Err(anyhow::anyhow!(
            "Block not indexed yet, latest block number: {}",
            latest_block_number
        ));
    }
    let request_url = format!("{}/{}/worker", url, block_number);
    let response = client.get(&request_url).send().await?.text().await;
    match response {
        Result::Ok(response) => Ok(response),
        Err(err) => Err(anyhow::anyhow!("Error getting worker url: {}", err)),
    }
}

pub fn get_block_query(block_number: u64) -> Value {
    let json_query = json!({
      "fromBlock": block_number,
      "toBlock": block_number,
      "fields": {
        "block": {
            "hash": true,

            "parentHash": true,
            "timestamp": true,
            "nonce": true,
            "sha3Uncles": true,
            "logsBloom": true,
            "transactionsRoot": true,
            "stateRoot": true,
            "receiptsRoot": true,
            "mixHash": true,
            "miner": true,
            "difficulty": true,
            "totalDifficulty": true,
            "extraData": true,
            "size": true,
            "gasLimit": true,
            "gasUsed": true,
            "baseFeePerGas": true,
                    }
        }
    });
    json_query
}
