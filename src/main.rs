use sha2::{Sha256};
use sha2::Digest;

#[derive(Debug)]
pub struct Block {
    inf: String,
    prev_hash:String, 
    hash:String,
}

fn sha256(inf: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(inf.as_bytes());
    let hex = hasher.finalize();
    format!("{:x}", hex)
}

fn add_block(prev_hash: String) -> Block {
    let inf = sha256(&prev_hash);
    let hash = sha256(&inf);
    let block= { Block {inf: inf, prev_hash: prev_hash, hash: hash} };
    block
}

fn add_genesis() -> Block {
    let inf = String::from("And there's no better sleep than a deep coma-UNNB");
    let hash = sha256(&inf);
    let prev_hash = hash.clone();
    let block = { Block {inf: inf, hash: hash, prev_hash: prev_hash} };
    block
}

fn main() {
    let mut blockchein: Vec<Block> = vec![];
    let genesis = add_genesis();
    blockchein.push(genesis);
    let mut count = 0;
    while count < 100 {
        let for_prev_hash = &blockchein[count];
        let block = add_block(for_prev_hash.hash.clone());
        blockchein.push(block);
        println!("{:#?}, {}", blockchein[count], count);
        count += 1;
    }
}

