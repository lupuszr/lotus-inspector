pub mod data;

use std::fmt;

use serde_json::{Value, json};
use serde_json::Value::Null;

use super::api::Connection;
use crate::lotus::api::{return_types};



#[derive(Debug)]
pub enum Methods {
    ChainHead,
    ChainGetTipSetByHeight
}

impl fmt::Display for Methods {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Filecoin.{:?}", self)
    }
}

pub struct Chain {
    conn: Connection
}


impl Chain {
    pub fn new(conn: Connection) -> Self {
        Chain{ conn }
    }

    pub async fn get_tipset_by_height(&self, height: i64) {
        let a = json!(height);
        let i1: i64 = 30;
        let i = json!(i1);
        let x: Vec<&Value> = vec![&i, &a, &Null];
        let v = self.conn.send(Methods::ChainGetTipSetByHeight.to_string(), x).await;

        match v {
            Ok(val) => {
                let x: Result<Value, serde_json::Error> = serde_json::from_value(val);
                println!("{:?}", x);
            },
            Err(err) => println!("{}", err),
        }

    }
    
    pub async fn head(&self) {
        let x: Vec<&String> = vec![];
        let v = self.conn.send(Methods::ChainHead.to_string(), x).await;

        match v {
            Ok(res) => {
                let x: Result<return_types::Root, serde_json::Error> = serde_json::from_value(res);
                let h = x.unwrap();
                // println!("{:?}", h.result.Blocks[0].Parents);
                println!("{:?}", h.result);
            },
            Err(err) => println!("{}", err),
        }
    }
    
}


#[cfg(test)]
mod tests {
    use crate::lotus::api::{Connection, Config};
    use crate::lotus::chain::Chain;

    #[tokio::test]
    async fn get_head() {
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
        let _x = Chain::new(con).head().await;
        
        // assert_eq!(con, 1);
    }

    #[tokio::test]
    async fn get_tipset_test() {
        let client = reqwest::Client::new();
        let url = format!(
            "http://{host}:1234/rpc/v0",
            host = "127.0.0.1"
        );
        /*
        TODO: import this from env
        */
        let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJBbGxvdyI6WyJyZWFkIiwid3JpdGUiLCJzaWduIiwiYWRtaW4iXX0.X4TlMMRX4BO8sMciIG3Z3ZNwAedUW_VACiK57ZpdFns";
        let conf = Config {
            client,
            url,
            token: token.to_string()
        };

        let con = Connection::new(conf);
        let x = Chain::new(con).get_tipset_by_height(1000000).await;
        
        // assert_eq!(con, 1);
    }
}