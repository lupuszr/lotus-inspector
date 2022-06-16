pub mod return_types;

use reqwest::{self, Client};
use serde::{Serialize, Deserialize};


pub struct Config {
    pub client: Client,
    pub token: String,
    pub url: String
}

#[derive(Debug, Serialize)]
struct LotusFormat<'a, T: Serialize + ?Sized> {
    jsonrpc: String,
    method: String,
    id: i64,
    // #[serde(rename = "userId")]
    params: Vec<&'a T>  //<serde_json::Value>,
}


pub struct Connection(Config);

impl Connection {
    pub fn new(c: Config) -> Self {
        Connection(c)
    }
    async fn send_basic<T>(&self, lotus_serialized_obj: &T) -> Result<serde_json::Value, reqwest::Error>
    where
        T: Serialize + ?Sized {
        let Config {client, token, url } = &self.0;
        let res: serde_json::Value = client.post(url)
                    .bearer_auth(token)
                    .header("Content-Type", "application/json")
                    .json(&lotus_serialized_obj)
                    .send()
                    .await?
                    .json()
                    .await?;            
        Ok(res)
    }

    pub async fn send<T>(&self, method: String, params: Vec<&T>) -> Result<serde_json::Value, reqwest::Error>
    where
        T: Serialize + ?Sized {
        let a = LotusFormat { jsonrpc: "2.0".to_string(), method, id: 1, params };
        // let _ = a.serialize(serde_json::value::Serializer)?;
        let x = self.send_basic(&a).await?;
        Ok(x)
    }
}


#[cfg(test)]
mod tests {
    use crate::lotus::api::{Connection, Config};

    #[test]
    fn make_connection() {
        let client = reqwest::Client::new();
        let url = format!(
            "http://{host}:1234/rpc/v0",
            host = "127.0.0.1"
        );
        /*
        TODO: import this from env
        */
        let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJBbGxvdyI6WyJyZWFkIiwid3JpdGUiLCJzaWduIiwiYWRtaW4iXX0.qyXAcjL2RLIC4Y46iWVjtnq8i2bpk-SWwO23nRdz038";
        let conf = Config {
            client,
            url,
            token: token.to_string()
        };

        let con = Connection::new(conf);
        
        // assert_eq!(con, 1);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}