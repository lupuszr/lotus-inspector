pub mod lotus;
// pub mod lab;

use reqwest::{self};
use serde::{Deserialize, Serialize};
use serde_json::{Number, json};

#[derive(Debug, Serialize, Deserialize)]
struct LotusBasic {
    jsonrpc: String,
    method: String,
    id: i64,
    // #[serde(rename = "userId")]
    params: Vec<serde_json::Value>,
}


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJBbGxvdyI6WyJyZWFkIiwid3JpdGUiLCJzaWduIiwiYWRtaW4iXX0.qyXAcjL2RLIC4Y46iWVjtnq8i2bpk-SWwO23nRdz038";

    let url = format!(
        "http://{host}:1234/rpc/v0",
        host = "127.0.0.1"
    );
    let client = reqwest::Client::new();

    let lotus_basic = LotusBasic {
        jsonrpc: "2.0".to_string(),
        method: "Filecoin.ChainGetTipSetByHeight".to_string(),
        id: 1,
        params: vec![json!(20),json!(null)]
    };
    let resp: serde_json::Value = client
                    .post(url)
                    .bearer_auth(token)
                    .header("Content-Type", "application/json")
                    .json(&lotus_basic)
                    .send()
                    .await?
                    .json()
                    .await?;
    println!("{}", resp);
    Ok(())
}
