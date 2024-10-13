use super::sqd_utils::{get_archive_url, get_block_query, get_worker_url, sqd_get_block_number};
use axum::http::response;
use reqwest::Client;
//get archive node url from env
use anyhow::{Error, Result};
use serde_json::{json, Value};

//Get relevant worker's url

pub async fn sqd_get_block_by_number(
    block_number: u64,
) -> Result<serde_json::Value, anyhow::Error> {
    let client = Client::new();
    let worker_url = get_worker_url(block_number).await?;
    let json_query = get_block_query(block_number);
    let response: String = client
        .post(worker_url)
        .json(&json_query)
        .send()
        .await?
        .text()
        .await?;
    let data: Value = serde_json::from_str(&response)?;
    let blocks = data.as_array();

    match blocks {
        Some(blocks) => Ok(blocks[0]["header"].to_owned()),
        None => Err(anyhow::anyhow!("Invalid JSON format: Expected an array")),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_get_current_block_number() {
        let block_number = sqd_get_block_number().await.unwrap();
        println!("block_number: {:?}", block_number);
        assert!(block_number > 0);
    }
    #[tokio::test]
    async fn test_get_worker_url() {
        let block_number = 1000000;
        let worker_url = get_worker_url(block_number).await.unwrap();
        println!("worker_url: {:?}", worker_url);
        assert!(worker_url.starts_with("https://"));
    }
    #[tokio::test]

    async fn test_get_block_by_number() {
        let block_number = 1000000;
        let block = sqd_get_block_by_number(block_number).await.unwrap();
        //println!("block: {:?}", block);
        assert!(block.is_object());
    }
}
