
pub struct Ticket {
    vrfp_proof: String
}

pub struct ElectionProof {
    vrfp_proof: String
}

pub struct WinPoStProofEntry {
    registered_proof: i32,
    proof_of_bytes: String
}

type ParentCid = Cid;


pub struct Blocks {
    miner: String,
    ticket: Ticket,
    election_proof: ElectionProof,
    win_PoSt_proof: Vec<WinPoStProofEntry>,
    parents: Vec<ParentCid>,
    parents_weight: String,
    height: i64,
    messages: Vec<Cid>
}



pub struct Cid {
    key: String,
    value: String
}

pub struct ChainHead {
    cids: Vec<Cid>,

}