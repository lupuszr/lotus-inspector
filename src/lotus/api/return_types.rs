#[derive(Debug, Serialize, Deserialize)]
pub struct Root {
    pub result: Result
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Result {
    pub Cids: Vec<HashMap<String, String>>,
    pub Blocks: Vec<Block>,
    pub Height: i64
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Ticket {
    #[serde(rename = "VRFProof")]
    vrfp_proof: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ElectionProof {
    #[serde(rename = "VRFProof")]
    pub vrfp_proof: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WinPoStProofEntry {
    #[serde(rename = "PoStProof")]
    post_proof: i64,
    #[serde(rename = "ProofBytes")]
    proof_bytes: String,
    #[serde(flatten)]
    unparsed_extra: Value
}

pub type ParentCid = Cid;
#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    Miner: String,
    Ticket: Ticket,
    ElectionProof: ElectionProof,
    #[serde(rename = "WinPoStProof")]
    WinPoStProof: Vec<WinPoStProofEntry>,
    pub Parents: Vec<HashMap<String, String>>,
    ParentWeight: String,
    Height: i64,
    // #[serde(flatten)]
    // Messages: Vec<Cid>

    Messages: HashMap<String, String>,
    BeaconEntries: Value,
    BLSAggregate: Value,
    ParentMessageReceipts: Value,
    Timestamp: i64,
    ForkSignaling: i32,
    ParentBaseFee: String,
    BlockSig: Value,
    ParentStateRoot: HashMap<String, String>,
}

// pub struct Cid {
//     key: String,
//     value: String
// }

use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_json::Value;
    
type Cid = HashMap<String, String>;
